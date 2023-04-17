use crate::codegen::apply::Codegen;
use crate::codegen::build_name;
use crate::ir::apply::{Tag, Term};
use crate::syntax;
use crate::syntax::*;

impl Codegen {
    pub fn build_match(&self, group: &RuleGroup, i: usize, parameter: Parameter) -> Term {
        use syntax::Parameter::*;

        let argument = Term::reference(&format!("arg{i}"));

        match parameter {
            U60(value) => Term::logical_and(
                Term::equal(Term::get_tag(argument.clone()), Term::Tag(Tag::U60)),
                Term::equal(Term::get_num(argument), Term::create_u60(value)),
            ),
            F60(value) => Term::logical_and(
                Term::equal(Term::get_tag(argument.clone()), Term::Tag(Tag::F60)),
                Term::equal(Term::get_num(argument), Term::create_f60(value)),
            ),
            Constructor(syntax::Constructor { name, .. }) => {
                let compiled_global_name = build_name(&name);
                let id = self
                    .global
                    .constructors
                    .get(&compiled_global_name)
                    .unwrap_or_else(|| panic!("no constructor for {}", compiled_global_name));

                Term::logical_and(
                    Term::equal(Term::get_tag(argument.clone()), Term::Tag(Tag::CONSTRUCTOR)),
                    Term::equal(Term::get_ext(argument), Term::ext(*id, &name)),
                )
            }
            Atom(..) if group.strict_parameters[i] => {
                // TODO: hoas for kind2

                Term::logical_or(
                    Term::equal(Term::get_tag(argument.clone()), Term::Tag(Tag::CONSTRUCTOR)),
                    Term::logical_or(
                        Term::equal(Term::get_tag(argument.clone()), Term::Tag(Tag::U60)),
                        Term::equal(Term::get_tag(argument), Term::Tag(Tag::F60)),
                    ),
                )
            }
            _ => Term::True,
        }
    }
}
