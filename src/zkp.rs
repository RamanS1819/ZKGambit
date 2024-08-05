use bellman::{
    Circuit, ConstraintSystem, SynthesisError,
    groth16
};
use bls12_381::{Bls12, Scalar};
// use ff::Field;
use rand::rngs::OsRng;

// Define the circuit
struct NumberMatchCircuit{
    secret: Option<u32>,
    guess: Option<u32>,
}

impl Circuit<Scalar> for NumberMatchCircuit {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        
        // Allocate the secret and guess asa private inputs
        let secret_var = cs.alloc(|| "secret", || {
            Ok(Scalar::from(self.secret.ok_or(SynthesisError::AssignmentMissing)? as u64))
        })?;

        let guess_var = cs.alloc(|| "guess",|| {
            Ok(Scalar::from(self.guess.ok_or(SynthesisError::AssignmentMissing)? as u64))
        })?;

        // Enforce the equality of the secret and guess
        // Constarint: secret = guess
        cs.enforce(
            || "secret == guess",
            |lc| lc +  secret_var,
            |lc| lc + CS::one(),
            |lc| lc + guess_var,
        );

        Ok(())
    }
}


// Setup the zk proof
pub fn setup() -> (
    groth16::Parameters<Bls12>,
    groth16::PreparedVerifyingKey<Bls12>,
) {
    let params  = {
        let c = NumberMatchCircuit {
            secret: None,
            guess: None,
        };
        groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng).unwrap()
    };

    let pvk = groth16::prepare_verifying_key(&params.vk);

    (params, pvk)
}


// Generate the proof
pub fn create_proof(
    params: &groth16::Parameters<Bls12>,
    secret: u32,
    guess: u32,
) -> groth16::Proof<Bls12> {

    let c = NumberMatchCircuit {
        secret: Some(secret),
        guess: Some(guess),
    };

    groth16::create_random_proof(c, params, &mut OsRng).unwrap()
}


// Verify the proof
pub fn verify_proof(
    pvk: &groth16::PreparedVerifyingKey<Bls12>,
    proof: &groth16::Proof<Bls12>,
) -> bool {
    let pubinput = vec![];

    groth16::verify_proof(pvk, &proof, &pubinput).is_ok()
}