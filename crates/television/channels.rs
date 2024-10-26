use crate::entry::Entry;
use color_eyre::eyre::Result;
use television_derive::{Broadcast, CliChannel, UnitChannel};

mod alias;
mod env;
mod files;
mod git_repos;
pub mod remote_control;
pub mod stdin;
mod text;

/// The interface that all television channels must implement.
///
/// # Important
/// The `OnAir` requires the `Send` trait to be implemented as
/// well. This is necessary to allow the channels to be used in a
/// multithreaded environment.
///
/// # Methods
/// - `find`: Find entries that match the given pattern. This method does not
///   return anything and instead typically stores the results internally for
///   later retrieval allowing to perform the search in the background while
///   incrementally polling the results.
///   ```rust
///   fn find(&mut self, pattern: &str);
///   ```
/// - `results`: Get the results of the search (at a given point in time, see
///   above). This method returns a specific portion of entries that match the
///   search pattern. The `num_entries` parameter specifies the number of
///   entries to return and the `offset` parameter specifies the starting index
///   of the entries to return.
///   ```rust
///   fn results(&mut self, num_entries: u32, offset: u32) -> Vec<Entry>;
///   ```
/// - `get_result`: Get a specific result by its index.
///   ```rust
///   fn get_result(&self, index: u32) -> Option<Entry>;
///   ```
/// - `result_count`: Get the number of results currently available.
///   ```rust
///   fn result_count(&self) -> u32;
///   ```
/// - `total_count`: Get the total number of entries currently available (e.g.
///   the haystack).
///   ```rust
///   fn total_count(&self) -> u32;
///   ```
///
pub trait OnAir: Send {
    /// Find entries that match the given pattern.
    ///
    /// This method does not return anything and instead typically stores the
    /// results internally for later retrieval allowing to perform the search
    /// in the background while incrementally polling the results with
    /// `results`.
    fn find(&mut self, pattern: &str);

    /// Get the results of the search (that are currently available).
    fn results(&mut self, num_entries: u32, offset: u32) -> Vec<Entry>;

    /// Get a specific result by its index.
    fn get_result(&self, index: u32) -> Option<Entry>;

    /// Get the number of results currently available.
    fn result_count(&self) -> u32;

    /// Get the total number of entries currently available.
    fn total_count(&self) -> u32;

    /// Check if the channel is currently running.
    fn running(&self) -> bool;

    /// Turn off
    fn shutdown(&self);
}

/// The available television channels.
///
/// Each channel is represented by a variant of the enum and should implement
/// the `OnAir` trait.
///
/// # Important
/// When adding a new channel, make sure to add a new variant to this enum and
/// implement the `OnAir` trait for it.
///
/// # Derive
/// The `CliChannel` derive macro generates the necessary glue code to
/// automatically create the corresponding `CliTvChannel` enum with unit
/// variants that can be used to select the channel from the command line.
/// It also generates the necessary glue code to automatically create a channel
/// instance from the selected CLI enum variant.
///
#[allow(dead_code, clippy::module_name_repetitions)]
#[derive(UnitChannel, CliChannel, Broadcast)]
pub enum TelevisionChannel {
    /// The environment variables channel.
    ///
    /// This channel allows to search through environment variables.
    Env(env::Channel),
    /// The files channel.
    ///
    /// This channel allows to search through files.
    Files(files::Channel),
    /// The git repositories channel.
    ///
    /// This channel allows to search through git repositories.
    GitRepos(git_repos::Channel),
    /// The text channel.
    ///
    /// This channel allows to search through the contents of text files.
    Text(text::Channel),
    /// The standard input channel.
    ///
    /// This channel allows to search through whatever is passed through stdin.
    Stdin(stdin::Channel),
    /// The alias channel.
    ///
    /// This channel allows to search through aliases.
    Alias(alias::Channel),
    /// The remote control channel.
    ///
    /// This channel allows to switch between different channels.
    RemoteControl(remote_control::RemoteControl),
}

/// NOTE: this could be generated by a derive macro
impl TryFrom<&Entry> for TelevisionChannel {
    type Error = String;

    fn try_from(entry: &Entry) -> Result<Self, Self::Error> {
        match entry.name.to_ascii_lowercase().as_ref() {
            "env" => Ok(TelevisionChannel::Env(env::Channel::default())),
            "files" => Ok(TelevisionChannel::Files(files::Channel::default())),
            "gitrepos" => {
                Ok(TelevisionChannel::GitRepos(git_repos::Channel::default()))
            }
            "text" => Ok(TelevisionChannel::Text(text::Channel::default())),
            "stdin" => Ok(TelevisionChannel::Stdin(stdin::Channel::default())),
            "alias" => Ok(TelevisionChannel::Alias(alias::Channel::default())),
            _ => Err(format!("Unknown channel: {}", entry.name)),
        }
    }
}
