# Deprecated alias for the Flake in packaging/. It keeps existing
# `github:alanfrigo/BetterLogi` inputs working while users move to
# `github:alanfrigo/BetterLogi?dir=packaging`; remove it in a later release.
#
# The alias declares nixpkgs only so `inputs.betterlogi.inputs.nixpkgs.follows`
# still reaches the package. It has no committed flake.lock (see .gitignore):
# packaging/flake.lock pins every other input. The relative path input needs
# Nix 2.26 or newer.
{
  description = "Deprecated alias for github:alanfrigo/BetterLogi?dir=packaging";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    betterlogi = {
      url = "path:./packaging";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { betterlogi, ... }:
    let
      moved = "the BetterLogi Flake moved: replace github:alanfrigo/BetterLogi with github:alanfrigo/BetterLogi?dir=packaging; the root alias will be removed in a later release";
    in
    builtins.warn moved betterlogi.outputs;
}
