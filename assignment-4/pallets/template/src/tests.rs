use crate::{mock::*, Error, Proofs, TNews};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(||{
		let claim = TNews {
			title: String::from("title 1").as_bytes().to_vec(),
			description: String::from("description 1").as_bytes().to_vec()
		};

        assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), (1, frame_system::Pallet::<Test>::block_number()))
    });
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
		let claim = TNews {
			title: String::from("title 1").as_bytes().to_vec(),
			description: String::from("description 1").as_bytes().to_vec()
		};

        let _ = TemplateModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            TemplateModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyClaimed
        );
    })
}

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = TNews {
			title: String::from("title 1").as_bytes().to_vec(),
			description: String::from("description 1").as_bytes().to_vec()
		};

        let _ = TemplateModule::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(TemplateModule::revoke_claim(Origin::signed(1), claim.clone()));
    })
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = TNews {
			title: String::from("title 1").as_bytes().to_vec(),
			description: String::from("description 1").as_bytes().to_vec()
		};

        assert_noop!(
            TemplateModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::NoSuchProof
        );
    })
}

#[test]
fn revoke_claim_failed_when_is_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = TNews {
			title: String::from("title 1").as_bytes().to_vec(),
			description: String::from("description 1").as_bytes().to_vec()
		};

        let _ = TemplateModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            TemplateModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotProofOwner
        );
    })
}

// Transferred
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = TNews {
			title: String::from("title 1").as_bytes().to_vec(),
			description: String::from("description 1").as_bytes().to_vec()
		};

        let _  = TemplateModule::create_claim(Origin::signed(1), claim.clone());

		assert_ok!(TemplateModule::transfer_claim(Origin::signed(1), 23, claim.clone()));

        assert_eq!(Proofs::<Test>::get(&claim), (23, frame_system::Pallet::<Test>::block_number()));

        assert_noop!(
            TemplateModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::NotProofOwner
        );
    })
}

#[test]
fn transfer_claim_failed_when_claim_no_exist() {
    new_test_ext().execute_with(|| {
        let claim = TNews {
			title: String::from("title 1").as_bytes().to_vec(),
			description: String::from("description 1").as_bytes().to_vec()
		};

        let _ = TemplateModule::create_claim(Origin::signed(1), claim.clone());

        let claim_temp =  TNews {
			title: String::from("title 2").as_bytes().to_vec(),
			description: String::from("description 2").as_bytes().to_vec()
		};

        assert_noop!(
            TemplateModule::transfer_claim(Origin::signed(1), 23, claim_temp.clone()),
            Error::<Test>::NoSuchProof
        );
    })
}

#[test]
fn transfer_claim_failed_when_sender_is_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = TNews {
			title: String::from("title 1").as_bytes().to_vec(),
			description: String::from("description 1").as_bytes().to_vec()
		};

        let _ = TemplateModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            TemplateModule::transfer_claim(Origin::signed(3), 23, claim.clone()),
            Error::<Test>::NotProofOwner
        );
    })
}
