{
  perSystem = { config, self', inputs', pkgs, ... }: {
    packages = {
      lightning-gui = pkgs.callPackage ./lightning-gui.nix { };
      default = self'.packages.lightning-gui;
    };
  };
}
