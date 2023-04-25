use fxhash::FxHashMap;

use crate::codegen::GlobalContext;
use crate::ir::apply::*;
use crate::ir::syntax::{Parameter, Rule};

pub type Insertion = Block;
pub type FreeIndex = u64;
pub type FreeArity = u64;
pub type FreeVec = Vec<(FreeIndex, FreeArity)>;

pub mod argument;
pub mod binary;
pub mod call;
pub mod collect;
pub mod deconstruct;
pub mod free;
pub mod graph;
pub mod group;
pub mod metadata;
pub mod term;
pub mod variable;

pub type Result<T> = std::result::Result<T, String>;

pub struct Codegen {
    lambdas: FxHashMap<u64, String>,
    global: Box<GlobalContext>,
    arguments: Vec<argument::Argument>,

    /// The variables that are used in this codegen, to be found within the
    /// [crate::ir::syntax::Atom] terms.
    ///
    /// TODO: This should be a [FxHashMap] instead of a [Vec]
    variables: Vec<(String, Term)>,

    /// The name index is used to generate unique names for the variables
    name_index: u64,

    /// The instructions that are generated by the codegen.
    instructions: Block,

    //>>>Constants section
    /// The extensions that are used in this codegen, to be displayed in the
    /// generated code/pretty print for debugging purposes.
    constant_extensions: FxHashMap<String, u64>,

    /// The tags that are used in this codegen, to be displayed in the
    /// generated code/pretty print for debugging purposes.
    constant_tags: FxHashMap<String, u64>,
    //<<<Constants section
}

impl Codegen {
    pub fn new(global: Box<GlobalContext>) -> Self {
        Self {
            global,
            name_index: 0,
            arguments: Vec::new(),
            lambdas: FxHashMap::default(),
            variables: Vec::new(),
            instructions: Block::default(),
            // constant sections
            constant_tags: FxHashMap::default(),
            constant_extensions: FxHashMap::default(),
        }
    }

    /// Creates a [Term::Tag] with the given [Tag], and adds it's tag id to the [Codegen].
    pub fn tag(&mut self, tag: Tag) -> Term {
        self.constant_tags.insert(tag.to_string(), tag.id());

        Term::Tag(tag)
    }

    /// Allocates a new [Term::Tag] with the given [Tag], and adds it's tag id to the [Codegen].
    pub fn alloc_tag(&mut self, tag: Tag) -> Term {
        self.constant_tags.insert(tag.to_string(), tag.id());

        Term::alloc(tag.size())
    }

    /// Creates a [Term::Ext] with the given [NameId], and adds it's extension id to the [Codegen].
    pub fn ext(&mut self, id: NameId, name: &str) -> Term {
        self.constant_extensions.insert(name.into(), id);

        Term::ext(id, name)
    }

    /// Allocates a new lambda, or returns the existing one.
    pub fn alloc_lam(&mut self, global_id: u64) -> String {
        if let Some(global_term) = self.lambdas.get(&global_id) {
            return global_term.clone();
        }

        let name = self.fresh_name("lam");
        self.instr(Instruction::binding(&name, Term::alloc(2)));

        if global_id != 0 {
            // FIXME: sanitizer still can't detect if a scope-less lambda doesn't use its bound
            //        variable, so we must write an Era() here. When it does, we can remove
            //        this line.
            self.instr(Instruction::link(Position::initial(&name), Term::erased()));
            self.lambdas.insert(global_id, name.clone());
        }

        name
    }

    pub fn alloc(&mut self, size: u64) -> Term {
        // TODO:
        // This will avoid calls to alloc() by reusing nodes from the left-hand side. Sadly, this seems
        // to decrease HVM's performance in some cases, probably because of added cache misses. Perhaps
        // this should be turned off. I'll decide later.

        Term::alloc(size)
    }

    /// Creates a new [Term::Agent] within a builder function [F], and
    /// adds it to the [Codegen], returning the [Term].
    pub fn make_agent<F: FnOnce(&mut Vec<Term>)>(&mut self, f: F) -> Term {
        let mut arguments = Vec::new();
        f(&mut arguments);

        Term::Agent(Agent {
            arity: arguments.len() as u64,
            arguments,
        })
    }

    pub fn build_link(&mut self, done: Term) {
        self.instructions
            .push(Instruction::link(Position::Host, done));
    }

    /// Adds a new instruction to the [Codegen].
    pub fn instr(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    /// Gets the [NameId] based on the name of the constructor, using
    /// the [GlobalContext] to get the index.
    pub fn get_name_id(&mut self, name: &str) -> NameId {
        let index = self
            .global
            .constructors
            .get(name)
            .unwrap_or_else(|| panic!("no constructor for {}", name));

        *index
    }

    /// Creates a new fresh name
    pub fn fresh_name(&mut self, _name: &str) -> String {
        let name = format!("{}", self.name_index);
        self.name_index += 1;
        name
    }

    fn build_constructor_patterns(&mut self, rule: &Rule) {
        let constructor_parameters = rule
            .parameters
            .iter()
            .enumerate()
            .filter(|parameter| matches!(parameter.1, Parameter::Constructor(..)));
        for (argument, parameter) in constructor_parameters {
            let Parameter::Constructor(constructor) = parameter else {
                continue;
            };

            for (index, _) in constructor.flatten_patterns.iter().enumerate() {
                let name = self.fresh_name("pat");
                let argument = self.get_argument(argument);
                let term = Term::load_arg(argument.clone().unbox(), index as u64);
                argument.add_field(Term::reference(&name));
                self.variables.push((name.clone(), Term::reference(&name)));
                self.instr(Instruction::binding(&name, term));
            }
        }
    }

    fn new_block(&self, instruction: Instruction) -> Self {
        Self {
            global: self.global.clone(),
            arguments: self.arguments.clone(),
            name_index: self.name_index,
            variables: self.variables.clone(),
            lambdas: self.lambdas.clone(),
            instructions: Block::with(instruction),
            // constant clonning
            constant_extensions: self.constant_extensions.clone(),
            constant_tags: self.constant_tags.clone(),
        }
    }
}
