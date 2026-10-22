{
  description = "BetterLogi — local-first companion for Logitech HID++ peripherals";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    # Workspace rust-version tracks current stable; nixpkgs' rustc lags it and
    # cargo then refuses to build. Same overlay devenv uses for the local toolchain.
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  # The dev shell lives in devenv.nix (devenv.yaml). This flake owns the Linux
  # package, its NixOS integration, checks, and formatter.
  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
      packageFor =
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system}.extend rust-overlay.overlays.default;
          toolchain = pkgs.rust-bin.stable.latest.minimal;
          rustPlatform = pkgs.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          };
        in
        pkgs.callPackage ./linux/package.nix {
          # The package builds the whole workspace, one level up. A Flake in a
          # subdirectory may read the rest of its source tree (NixOS/nix#4414).
          src = ../.;
          inherit rustPlatform;
        };
      moduleCheckFor =
        system:
        let
          lib = nixpkgs.lib;
          pkgs = nixpkgs.legacyPackages.${system};
          package = self.packages.${system}.betterlogi;
          evaluate =
            launchAtLogin:
            (lib.nixosSystem {
              inherit system;
              modules = [
                self.nixosModules.default
                {
                  programs.betterlogi = {
                    enable = true;
                    inherit launchAtLogin;
                  };
                  system.stateVersion = "26.05";
                }
              ];
            }).config;
          enabled = evaluate true;
          manual = evaluate false;
        in
        assert lib.elem package enabled.environment.systemPackages;
        assert lib.elem package enabled.services.udev.packages;
        assert enabled.systemd.user.services.betterlogi-agent.wantedBy == [ "graphical-session.target" ];
        assert manual.systemd.user.services.betterlogi-agent.wantedBy == [ ];
        assert enabled.systemd.user.services.betterlogi-agent.after == [ "graphical-session.target" ];
        assert enabled.systemd.user.services.betterlogi-agent.partOf == [ "graphical-session.target" ];
        assert manual.systemd.user.services.betterlogi-agent.partOf == [ ];
        assert
          enabled.systemd.user.services.betterlogi-agent.serviceConfig.ExecStart
          == "${package}/bin/betterlogi-agent";
        pkgs.runCommand "betterlogi-nixos-module-check" { } ''
          touch "$out"
        '';
    in
    {
      packages = forAllSystems (
        system:
        let
          betterlogi = packageFor system;
        in
        {
          inherit betterlogi;
          default = betterlogi;
        }
      );

      checks = forAllSystems (system: {
        package = self.packages.${system}.betterlogi;
        nixos-module = moduleCheckFor system;
      });

      formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.nixfmt);

      nixosModules = {
        betterlogi =
          {
            lib,
            pkgs,
            ...
          }:
          {
            imports = [ ./linux/nixos-module.nix ];
            programs.betterlogi.package = lib.mkDefault self.packages.${pkgs.stdenv.hostPlatform.system}.betterlogi;
          };
        default = self.nixosModules.betterlogi;
      };
    };
}
