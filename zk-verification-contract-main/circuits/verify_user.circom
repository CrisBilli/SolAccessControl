pragma circom 2.0.0;
include "../node_modules/circomlib/circuits/poseidon.circom";

template PersonalInfo() {
  // Hash the private inputs using Poseidon
  component userRoleHasher = Poseidon(15);
  component locationHasher = Poseidon(15);

  signal input userRole[15];          // Private input: userRole  
  signal input location[15];          // Private input: location

  userRoleHasher.inputs <== userRole;
  locationHasher.inputs <== location;

  signal output userRole_out;
  signal output location_out;

  userRole_out <== userRoleHasher.out;
  location_out <== locationHasher.out;

  // Enforce that the hashed private inputs match the constant expected hashes
  12190118236461756001683974151677789172651763991449464096001596654482483488634 === userRole_out;
  3112626621302619800867442836869779507811667491815112894613862090609351071117 === location_out;
}

component main = PersonalInfo();