{
	description = "TODO document this";

	inputs.nixpkgs.url = "nixpkgs/nixos-24.11-small";

	outputs = { self, nixpkgs }: let
		pkgs = import nixpkgs { system = "x86_64-linux"; };
		rustPlatform = pkgs.rustPlatform;
		autolock = rustPlatform.buildRustPackage {
			pname = "autolock";

			version = "0.1.0";

			src = ./.;

			buildInputs = [
			];

			checkFlags = [
				# Can be used to skip failing tests
				# "--skip=api::...::test_simple_5m"
			];

			cargoLock.lockFile = ./Cargo.lock;
		};
	in {
		devShells.${pkgs.system}.default = pkgs.mkShell {
			buildInputs = [
				pkgs.cargo
				pkgs.cargo-flamegraph
				pkgs.lldb
				pkgs.rustc
				pkgs.rustfmt
			];
		};

		packages.${pkgs.system}.default = autolock;

		nixos_options =
			{ config, lib, pkgs, ... }:

			let
				cfg = config.programs.autolock;
			in
			{
				options = {
					programs.autolock = {
						enable = lib.mkEnableOption "TODO document this";

						user_agent = lib.mkOption {
							type = lib.types.str;
							description = "Sample option";
						};
					};
				};

				config = lib.mkMerge [
					(lib.mkIf cfg.enable {
						environment.systemPackages = [ autolock ];
					})
				];
			};
	};
}
