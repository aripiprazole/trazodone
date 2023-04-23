use crate::codegen::apply::Codegen;
use crate::codegen::build_name;
use crate::ir::apply::{Tag, Term};
use crate::ir::syntax;
use crate::ir::syntax::*;

impl Codegen {
    pub fn build_match(&mut self, group: &RuleGroup, i: usize, parameter: Parameter) -> Term {
        use crate::ir::syntax::Parameter::*;

        let argument = Term::reference(&format!("arg{i}"));

        match parameter {
            U60(value) => Term::logical_and(
                Term::equal(Term::get_tag(argument.clone()), self.tag(Tag::U60)),
                Term::equal(Term::get_num(argument), Term::create_u60(value)),
            ),
            F60(value) => Term::logical_and(
                Term::equal(Term::get_tag(argument.clone()), self.tag(Tag::F60)),
                Term::equal(Term::get_num(argument), Term::create_f60(value)),
            ),
            Constructor(syntax::Constructor { name, .. }) => {
                let id = self.get_name_id(&name);

                Term::logical_and(
                    Term::equal(Term::get_tag(argument.clone()), self.tag(Tag::CONSTRUCTOR)),
                    Term::equal(Term::get_ext(argument), self.ext(id, &name)),
                )
            }
            Atom(..) if group.strict_parameters[i] => {
                // TODO: hoas for kind2

                Term::logical_or(
                    Term::equal(Term::get_tag(argument.clone()), self.tag(Tag::CONSTRUCTOR)),
                    Term::logical_or(
                        Term::equal(Term::get_tag(argument.clone()), self.tag(Tag::U60)),
                        Term::equal(Term::get_tag(argument), self.tag(Tag::F60)),
                    ),
                )
            }
            _ => Term::True,
        }
    }
}
