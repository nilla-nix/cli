import kleur from "kleur";
import project from "~/util/help/project";
import common from "~/util/help/common";
import sources from "~/util/help/sources";

export default function help() {
	// prettier-ignore
	const message = `
${kleur.bold("🍦 Nilla - build")}

${kleur.bold("DESCRIPTION")}

  Build a package in a Nilla project.

${kleur.bold("USAGE")}

  ${kleur.dim("$")} ${kleur.bold("nilla")} build <name> [options]

${kleur.bold("OPTIONS")}

${project}

${common}

${kleur.bold("EXAMPLES")}

  ${kleur.dim("# Build a package from a local Nilla project.")}
  ${kleur.dim("$")} ${kleur.bold("nilla")} build mypackage

  ${kleur.dim("# Build a package from a Nilla project on GitHub.")}
  ${kleur.dim("$")} ${kleur.bold("nilla")} build mypackage --project github:myuser/myrepo

  ${kleur.dim("# Build a package from a Nilla project in a tarball.")}
  ${kleur.dim("$")} ${kleur.bold("nilla")} build mypackage --project https://example.com/myproject.tar.gz

${sources}
	`;

	console.log(message);
}
