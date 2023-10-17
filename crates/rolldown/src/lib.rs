mod bundler;
mod plugin;

use std::sync::Arc;

use rolldown_resolver::Resolver;

pub(crate) type SharedResolver = Arc<Resolver>;
pub type BuildError = rolldown_error::Error;
pub type BuildResult<T> = Result<T, Box<BuildError>>;

pub use crate::{
  bundler::{
    bundle::asset::Asset,
    bundler::Bundler,
    options::{
      input_options::{InputItem, InputOptions},
      output_options::OutputOptions,
    },
  },
  plugin::{
    args::{HookLoadArgs, HookResolveIdArgs, HookTransformArgs},
    context::PluginContext,
    output::{HookLoadOutput, HookResolveIdOutput},
    plugin::{HookLoadReturn, HookResolveIdReturn, HookTransformReturn, Plugin},
  },
};