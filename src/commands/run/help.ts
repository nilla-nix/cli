import kleur from "kleur";
import sources from "~/util/help/sources";
import common from "~/util/help/common";
import project from "~/util/help/project";

export default function help() {
	// prettier-ignore
	const message = `
${kleur.bold("🍦 Nilla - run")}

${kleur.bold("DESCRIPTION")}

  Run a package's main program.

${kleur.bold("USAGE")}

  ${kleur.dim("$")} ${kleur.bold("nilla")} run <name> [options]

${kleur.bold("OPTIONS")}

${project}

${common}

${kleur.bold("EXAMPLES")}

  ${kleur.dim("# Run the default package in a Nilla project on GitHub.")}
  ${kleur.dim("$")} ${kleur.bold("nilla")} run --project github:myuser/myrepo

  ${kleur.dim("# Run a specific package in a local Nilla project.")}
  ${kleur.dim("$")} ${kleur.bold("nilla")} run mypackage

  ${kleur.dim(`# Supply arguments to the package's main program using "--" followed by your arguments.`)}
  ${kleur.dim("$")} ${kleur.bold("nilla")} run mypackage -- --my-arg

${sources}
	`;

	console.log(message);
}
