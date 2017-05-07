
pub static OPTIONS: &'static str = r##"
  -?, --help         show list of command-line options
  -v, --version      show version of exa

DISPLAY OPTIONS
  -1, --oneline      display one entry per line
  -l, --long         display extended file metadata as a table
  -G, --grid         display entries as a grid (default)
  -x, --across       sort the grid across, rather than downwards
  -R, --recurse      recurse into directories
  -T, --tree         recurse into directories as a tree
  -F, --classify     display type indicator by file names
  --colo[u]r=WHEN    when to use terminal colours (always, auto, never)
  --colo[u]r-scale   highlight levels of file sizes distinctly

FILTERING AND SORTING OPTIONS
  -a, --all                  don't hide hidden and 'dot' files
  -d, --list-dirs            list directories like regular files
  -r, --reverse              reverse the sort order
  -s, --sort SORT_FIELD      which field to sort by:
  --group-directories-first  list directories before other files
  -I, --ignore-glob GLOBS    glob patterns (pipe-separated) of files to ignore
  Valid sort fields:         name, Name, extension, Extension, size,
                             modified, accessed, created, inode, none

"##;

pub static LONG_OPTIONS: &'static str = r##"
LONG VIEW OPTIONS
  -b, --binary       list file sizes with binary prefixes
  -B, --bytes        list file sizes in bytes, without any prefixes
  -g, --group        list each file's group
  -h, --header       add a header row to each column
  -H, --links        list each file's number of hard links
  -i, --inode        list each file's inode number
  -L, --level DEPTH  limit the depth of recursion
  -m, --modified     use the modified timestamp field
  -S, --blocks       show number of file system blocks
  -t, --time FIELD   which timestamp field to list (modified, accessed, created)
  -u, --accessed     use the accessed timestamp field
  -U, --created      use the created timestamp field
"##;

pub static GIT_HELP:      &'static str = r##"  --git              list each file's Git status, if tracked"##;
pub static EXTENDED_HELP: &'static str = r##"  -@, --extended     list each file's extended attributes and sizes"##;
