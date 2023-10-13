use crate::term::{DefNames, DefinitionBook, Name, Term};
use std::collections::HashMap;

impl DefinitionBook {
  /// Decides if names inside a term belong to a Var or to a Ref.
  /// Precondition: Refs are encoded as vars.
  /// Postcondition: Refs are encoded as refs, with the correct def id.
  pub fn resolve_refs(&mut self) {
    for (_, def) in &mut self.defs {
      def.body.resolve_refs(&self.def_names);
    }
  }
}

impl Term {
  pub fn resolve_refs(&mut self, def_names: &DefNames) {
    resolve_refs(self, def_names, &mut HashMap::new())
  }
}

fn resolve_refs(term: &mut Term, def_names: &DefNames, scope: &mut HashMap<Name, usize>) {
  match term {
    Term::Lam { nam, bod } => {
      push_scope(nam.clone(), scope);
      resolve_refs(bod, def_names, scope);
      pop_scope(nam.clone(), scope);
    }
    Term::Let { nam, val, nxt } => {
      resolve_refs(val, def_names, scope);
      push_scope(Some(nam.clone()), scope);
      resolve_refs(nxt, def_names, scope);
      pop_scope(Some(nam.clone()), scope);
    }
    Term::Dup { fst, snd, val, nxt } => {
      resolve_refs(val, def_names, scope);
      push_scope(fst.clone(), scope);
      push_scope(snd.clone(), scope);
      resolve_refs(nxt, def_names, scope);
      pop_scope(fst.clone(), scope);
      pop_scope(snd.clone(), scope);
    }

    // If variable not defined, we check if it's a ref and swap if it is.
    Term::Var { nam } => {
      if is_var_in_scope(nam.clone(), scope) {
        if let Some(def_id) = def_names.def_id(nam) {
          *term = Term::Ref { def_id };
        }
      }
    }
    Term::Chn { bod, .. } => resolve_refs(bod, def_names, scope),
    Term::App { fun, arg } => {
      resolve_refs(fun, def_names, scope);
      resolve_refs(arg, def_names, scope);
    }
    Term::Match { cond, zero, succ } => {
      resolve_refs(cond, def_names, scope);
      resolve_refs(zero, def_names, scope);
      resolve_refs(succ, def_names, scope);
    }
    Term::Sup { fst, snd } => {
      resolve_refs(fst, def_names, scope);
      resolve_refs(snd, def_names, scope);
    }
    Term::Opx { fst, snd, .. } => {
      resolve_refs(fst, def_names, scope);
      resolve_refs(snd, def_names, scope);
    }
    Term::Lnk { .. } => (),
    // We don't expect to find any refs here, but if we do, just ignore it.
    Term::Ref { .. } => (),
    Term::Era => (),
    Term::Num { .. } => (),
  }
}

fn push_scope(name: Option<Name>, scope: &mut HashMap<Name, usize>) {
  if let Some(name) = name {
    let var_scope = scope.entry(name.clone()).or_default();
    *var_scope += 1;
  }
}

fn pop_scope(name: Option<Name>, scope: &mut HashMap<Name, usize>) {
  if let Some(name) = name {
    let var_scope = scope.entry(name.clone()).or_default();
    *var_scope -= 1;
  }
}

fn is_var_in_scope(name: Name, scope: &mut HashMap<Name, usize>) -> bool {
  *scope.entry(name.clone()).or_default() == 0
}
