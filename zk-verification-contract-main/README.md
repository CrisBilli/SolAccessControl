## ACCESS VERIFICATION CONTRACT


solana version 1.15.2


anchor version 0.26.0

``
  479  circom verify_user.circom --r1cs --wasm --sym --c
  480  cd verify_user_js/
  483  node generate_witness.js verify_user.wasm ../input.json witness.wtns
  484  wget https://storage.googleapis.com/zkevm/ptau/powersOfTau28_hez_final_15.ptau
  485  cd ..
  486  snarkjs groth16 setup ./verify_user.r1cs powersOfTau28_hez_final_15.ptau verify_user_0000.zkey
  487  snarkjs zkey contribute verify_user_0000.zkey verify_user_0001.zkey --name="1st Contributor Name" -v
  488  snarkjs zkey export verificationkey verify_user_0001.zkey verification_key.json
  490  snarkjs groth16 prove verify_user_0001.zkey ./verify_user_js/witness.wtns proof.json public.json
  491  snarkjs groth16 verify verification_key.json public.json proof.json
  ``