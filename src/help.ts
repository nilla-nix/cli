import kleur from "kleur";
import common from "~/util/help/common";

export default function help() {
	// prettier-ignore
	const message = `
${kleur.bold("🍦 Nilla")}

${kleur.bold("DESCRIPTION")}

  Create and manage packages, systems, and more with Nix.

${kleur.bold("USAGE")}

  ${kleur.dim("$")} ${kleur.bold("nilla")} <command> [options]

${kleur.bold("COMMANDS")}

  run                       Run a package
  build                     Build a package
  shell                     Start a development shell
  nixos                     Manage a NixOS system

${kleur.bold("OPTIONS")}

${common}

${kleur.bold("EXAMPLES")}

  ${kleur.dim("# Run a package from a local Nilla project.")}
  ${kleur.dim("$")} ${kleur.bold("nilla")} run mypackage

  ${kleur.dim("# Build a package from a Nilla project on GitHub.")}
  ${kleur.dim("$")} ${kleur.bold("nilla")} build mypackage --project github:myuser/myrepo

  ${kleur.dim("# Start a development shell from a Nilla project in another directory.")}
  ${kleur.dim("$")} ${kleur.bold("nilla")} shell myshell --project ~/myproject

  ${kleur.dim("# Build and switch to a NixOS configuration in a local Nilla project.")}
  ${kleur.dim("$")} ${kleur.bold("nilla")} nixos switch mysystem
	`;

	console.log(message);
}
