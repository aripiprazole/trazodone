use crate::codegen::apply::Codegen;
use crate::ir::apply::{Tag, Term};
use crate::ir::syntax;
use crate::ir::syntax::*;

impl Codegen {
    pub fn build_match(&mut self, group: &RuleGroup, i: usize, parameter: Parameter) -> Term {
        use crate::ir::syntax::Parameter::*;

        let argument = self.get_argument(i).unbox();

        match parameter {
            U60(value) => Term::logical_and(
                Term::equal(argument.get_tag(), self.tag(Tag::U60)),
                Term::equal(argument.get_num(), Term::create_u60(value)),
            ),
            F60(value) => Term::logical_and(
                Term::equal(argument.get_tag(), self.tag(Tag::F60)),
                Term::equal(argument.get_num(), Term::create_f60(value)),
            ),
            Constructor(syntax::Constructor { name, .. }) => {
                let id = self.get_name_id(&name);

                Term::logical_and(
                    Term::equal(argument.get_tag(), self.tag(Tag::CONSTRUCTOR)),
                    Term::equal(argument.get_ext(), self.ext(id, &name)),
                )
            }
            Atom(..) if group.strict_parameters[i] => {
                // TODO: hoas for kind2

                Term::logical_or(
                    Term::equal(argument.get_tag(), self.tag(Tag::CONSTRUCTOR)),
                    Term::logical_or(
                        Term::equal(argument.get_tag(), self.tag(Tag::U60)),
                        Term::equal(argument.get_tag(), self.tag(Tag::F60)),
                    ),
                )
            }
            _ => Term::True,
        }
    }
}
