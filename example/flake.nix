{
  inputs = {
    xnode-manager.url = "github:Openmesh-Network/xnode-manager";
    pnl-calculator.url = "github:OpenxAI-Network/pnl-calculator"; # "path:..";
    nixpkgs.follows = "pnl-calculator/nixpkgs";
  };

  outputs = inputs: {
    nixosConfigurations.container = inputs.nixpkgs.lib.nixosSystem {
      specialArgs = {
        inherit inputs;
      };
      modules = [
        inputs.xnode-manager.nixosModules.container
        {
          services.xnode-container.xnode-config = {
            host-platform = ./xnode-config/host-platform;
            state-version = ./xnode-config/state-version;
            hostname = ./xnode-config/hostname;
          };
        }
        inputs.pnl-calculator.nixosModules.default
        {
          services.pnl-calculator.enable = true;
        }
      ];
    };
  };
}
