import kleur from "kleur";

const message = `
${kleur.bold("SOURCES")}

  path

    Fetch a Nilla project from a file path. This follows the format:

      path:<path>

  git

    Fetch a Nilla project from a Git repository. This follows the format:

      git:<url>

    Optionally, additional customization can be applied using query parameters:

      git:<url>?rev=<rev>&ref=<ref>&submodules=true&dir=<dir>

  github

    Fetch a Nilla project from a GitHub repository. This follows the format:

      github:<owner>/<repo>

    Optionally, additional customization can be applied using query parameters:

      github:<owner>/<repo>?rev=<rev>&dir=<dir>

  gitlab

    Fetch a Nilla project from a GitLab repository. This follows the format:

      gitlab:<owner>/<repo>

    Optionally, additional customization can be applied using query parameters:

      gitlab:<owner>/<repo>?rev=<rev>&dir=<dir>

  sourcehut

    Fetch a Nilla project from a Sourcehut repository. This follows the format:

      sourcehut:<owner>/<repo>

    Optionally, additional customization can be applied using query parameters:

      sourcehut:<owner>/<repo>?rev=<rev>&dir=<dir>

  tarball

    Fetch a Nilla project from a tarball. This follows the format:

      tarball:<url>

    The tarball source is also the default used when no other protocol matches. For
    example, the following are equivalent:

      tarball:http://example.com/project.tar.gz

      http://example.com/project.tar.gz
`.trim();

export default message;
