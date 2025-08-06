---
title: Post Template Generator
tagline: An Hour of Engineering to Save Myself Seconds
series: Orpheus Architecture
tags:
  - projects
  - programming
---

I've written a good handful of posts for this site now, and I've discovered a
few rough edges that are still a part of that process. The biggest one pops up
right at the start when I create a blank post. Lyre (rightfully) complains that
the post lacks some expected elements, like metadata and _any content at all_. I
like to leave `just author` running while I write so I can see updates live, and
errors in the terminal to remind me of what I'm missing.

The errors are helpful in guiding me towards completing the document (but they
could be more helpful, I'm working on that). However, most of the metadata is
optional, so Lyre doesn't complain about them. This means I often forget about
the metadata options, or have to go check code or other posts to see what the
options are. If I add new metadata options in the future, they won't be present
in old posts for me to copy. I'd prefer to have all the options visible right
away, and leave them blank if I want. That'll slightly reduce the cognitive load
of starting a post, and let me focus more on actually writing the content.

Additionally, it's just a lot of boilerplate to repeat every time. Well, _a lot_
is probably hyperbolic, but it's not nothing. There's no reason to do this
manually when it's so trivial to automate!

The solution: Add blank post generation as a Lyre feature.

# Adding Subcommands to Lyre

Lyre was made as a very simple tool. It's just an executable binary that does
its tasks and logs success/failure states. If I wanted to add alternate behavior
to it, I needed to make it a little more sophisticated.

[Clap](https://docs.rs/clap/latest/clap/) is a popular crate for handling
Command Line Interface arguments. It can generate parsing and validation, and
even generate help commands. All I've gotta do is declaratively define the
structure of what the parsed arguments should look like.

It feels perhaps a bit overkill to use clap for this. After all, I'm adding just
one new behavior to Lyre. However, if I want to add more in the future, I'll be
glad I used this to pave the way for it.

So, I used clap to add a `build` command, and made that the default behavior to
match the previous usage of Lyre:

```rs
#[derive(Parser)]
#[command(version, about, infer_subcommands = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Default)]
enum Commands {
    /// Default command. Builds all Orpheus content
    #[default]
    Build,
}
```

> **Note:**  
> I set `infer_subcommands` to `true` in the above example. It's pretty neat. Tt
> allows clap to figure out your intended subcommand based on as little text as
> possible. So, for example, `lyre b` is equivalent to `lyre build`.

I then shifted Lyre's old main behavior behind a match on the selected command:

```rs
let args = Args::parse();

match args.command {
    Commands::Build => {
        let started = SystemTime::now();

        let mut state = melody::prepare()?;

        <web::Parcel as Melody>::conduct(&mut state)?;
        <web::Favicon as Melody>::conduct(&mut state)?;
        <web::SCSS as Melody>::conduct(&mut state)?;
        <web::Images as Melody>::conduct(&mut state)?;
        <posts::Posts as Melody>::conduct(&mut state)?;
        <pages::Pages as Melody>::conduct(&mut state)?;

        finalize(state)?;

        println!(
            "{} in {:?}",
            "Lyre Completed".green(),
            started.elapsed()?.yellow()
        );
    }
}
```

With that in place, I can add more commands to the `Commands` enum and add more
branches to the match statement, and that's all that's needed to add more
commands.

# Adding the Gen Command

So let's add a new command right away:

```rs
#[derive(Subcommand, Default)]
enum Commands {
    /// Default command. Builds all Orpheus content
    #[default]
    Build,

    // 👇 new stuff
    /// Generate new empty content files from templates
    Gen {
        #[command(subcommand)]
        template: Templates,
    },
}

// 👇 new stuff
#[derive(Subcommand)]
enum Templates {
    /// Generates an empty post
    Post {
        /// Title of the post. Also used to generate the file name
        #[arg(num_args(..), trailing_var_arg(true),)]
        title: Vec<String>,
    },
}

// ...

let args = Args::parse();

match args.command {
    Commands::Build => {
      // ...
    }
    // 👇 new stuff
    Commands::Gen { template } => match template {
        Templates::Post { title } => {
            let title = title.join(" ");
            let frontmatter = Frontmatter {
                title: title.clone(),
                published: Utc::now().format("%F").to_string(),
                ..Default::default()
            };

            let mut path = Path::new("content").join("posts").to_path_buf();

            path = path.join(slugify(title)).with_extension("md");

            fs::create_dir_all(path.with_file_name(""))?;
            fs::write(
                &path,
                format!(
                    "---\n{}---\n\n# Content Here\n",
                    serde_yaml::to_string(&frontmatter)?.replace(" null", "")
                ),
            )?;

            println!("Generated at: {}", path.to_str().unwrap().blue());
        }
    },
}
```

This one is a little more complicated. We've got a command called `Gen`, and it
has a subcommand `Post`. I structured it in this way so that I could add other
templates in the future. The `Post` command takes in a `Vec<String>` and
consumes the rest of the command line.

In practice, this means that

```sh
$ lyre gen post My Title Here
```

gets parsed as

```rs
Commands::Gen{ template: Templates::Post{ title: vec!["My", "Title", "Here"] } }
```

which we can easily destruct with our match statement.

Getting the post title from the commandline is important, because the filename
is based on the post title.

Inside that match statement, we generate a new `Frontmatter` from default, and
overwrite the `title` and `published` fields with data we've got available now.
The frontmatter gets serialized into yaml, and then saved into a very simple
template. By generating a full `Frontmatter` like this, we can ensure that any
new fields that are added to `Frontmatter` get added to all new posts.

At the end of the process, we output the path to the generated file. In my
environment, I can `ctrl+click` this path to open up that file easily. If your
environment doesn't support this, it's at least easy to copy.

This is good! It works!

# Adding Series Entry

It's got one inconvenience: I've started containing each series of posts within
a separate subdirectory as a convention. The current post generation places them
all within the base `posts` directory. So if the post is part of a series, I
have to write the series into the frontmatter myself and then manually move the
file to the correct subdirectory (possibly creating that subdirectory if this is
a new series). It would be convenient if we could read series from the CLI both
save it to the frontmatter and select the correct directory.

Thankfully, with the way things are set up, this is an easy addition. However,
there's some interface design work to be done.

I first considered something like this:

```sh
$ lyre gen post --series "My Series Name" My Title Here
```

This was _fine_, but a little wordy. I also generally find quotes in CLIs give
me the ick, even if they are frequently necessary. However, I think we can do
better for this case. This tool isn't really designed to be called by other
tools, only by humans. So it's okay if all the inputs and outputs aren't
directly part of the interface. We can turn this into a (very) simple TUI:

```sh
$ lyre gen post My Title Here
Series (leave blank for none): My Series Name
```

The user will be prompted to enter a series name if they want one. If not, they
can simply hit enter and skip it. This seems pretty flexible, and I could add
more to it in the future. Perhaps I could even turn it into a real TUI menu if I
really wanted to lean into the overkill even more.

Here's the code I used to implement this:

```rs
Templates::Post { title } => {
  // 👇 new stuff
  print!("Series (leave blank for none): ");
  stdout().flush()?;
  let mut series = String::new();
  stdin().read_line(&mut series)?;
  series = series.trim().to_string();

  // 👇 new stuff
  let series = if !series.is_empty() {
      Some(series)
  } else {
      None
  };

  let title = title.join(" ");
  let frontmatter = Frontmatter {
      title: title.clone(),
      series: series.clone(), // 👈 new line
      published: Utc::now().format("%F").to_string(),
      ..Default::default()
  };

  let mut path = Path::new("content").join("posts").to_path_buf();

  // 👇 new stuff
  if let Some(series) = series {
      path = path.join(slugify(series));
  }

  path = path.join(slugify(title)).with_extension("md");

  fs::create_dir_all(path.with_file_name(""))?;
  fs::write(
      &path,
      format!(
          "---\n{}---\n\n# Content Here\n",
          serde_yaml::to_string(&frontmatter)?.replace(" null", "")
      ),
  )?;

  println!("Generated at: {}", path.to_str().unwrap().blue());
}
```

It only takes a handful of lines to prompt the user for input. `Frontmatter`
expects `series` to be an `Option`, so we wrap it in one with blank becoming
`None`. We also inject the series slug into the file path to select the
subdirectory.

Done! Lyre can now generate post templates for me, with plenty of room for more
CLI features in the future.
