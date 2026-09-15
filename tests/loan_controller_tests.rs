use actix_web::{test, web, App, http::{StatusCode, Method}};
use serde_json::json;

mod integration_tests {
    use super::*;

    #[actix_rt::test]
    async fn test_create_loan_success() {
        let app = test::init_service(
            App::new()
                .route("/api/loans", web::post().to(crate::controllers::loan_controller::create_loan))
        ).await;

        let payload = json!({
            "amount": 10000.0,
            "interest_rate": 5.5,
            "due_date": "2024-12-31"
        });

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_rt::test]
    async fn test_create_loan_invalid_amount() {
        let app = test::init_service(
            App::new()
                .route("/api/loans", web::post().to(crate::controllers::loan_controller::create_loan))
        ).await;

        let payload = json!({
            "amount": -1000.0,
            "interest_rate": 5.5,
            "due_date": "2024-12-31"
        });

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_create_loan_invalid_interest_rate() {
        let app = test::init_service(
            App::new()
                .route("/api/loans", web::post().to(crate::controllers::loan_controller::create_loan))
        ).await;

        let payload = json!({
            "amount": 10000.0,
            "interest_rate": -5.0,
            "due_date": "2024-12-31"
        });

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_create_loan_invalid_date_format() {
        let app = test::init_service(
            App::new()
                .route("/api/loans", web::post().to(crate::controllers::loan_controller::create_loan))
        ).await;

        let payload = json!({
            "amount": 10000.0,
            "interest_rate": 5.5,
            "due_date": "invalid-date"
        });

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_get_loan_by_id_success() {
        let app = test::init_service(
            App::new()
                .route("/api/loans/{id}", web::get().to(crate::controllers::loan_controller::get_loan))
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/loans/1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_loan_by_id_not_found() {
        let app = test::init_service(
            App::new()
                .route("/api/loans/{id}", web::get().to(crate::controllers::loan_controller::get_loan))
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/loans/99999")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_list_loans_pagination() {
        let app = test::init_service(
            App::new()
                .route("/api/loans", web::get().to(crate::controllers::loan_controller::list_loans))
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/loans?page=1&per_page=10")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_list_loans_default_pagination() {
        let app = test::init_service(
            App::new()
                .route("/api/loans", web::get().to(crate::controllers::loan_controller::list_loans))
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/loans")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_update_loan_success() {
        let app = test::init_service(
            App::new()
                .route("/api/loans/{id}", web::put().to(crate::controllers::loan_controller::update_loan))
        ).await;

        let payload = json!({
            "amount": 15000.0,
            "interest_rate": 6.0,
            "due_date": "2025-06-30"
        });

        let req = test::TestRequest::put()
            .uri("/api/loans/1")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_update_loan_not_found() {
        let app = test::init_service(
            App::new()
                .route("/api/loans/{id}", web::put().to(crate::controllers::loan_controller::update_loan))
        ).await;

        let payload = json!({
            "amount": 15000.0,
            "interest_rate": 6.0,
            "due_date": "2025-06-30"
        });

        let req = test::TestRequest::put()
            .uri("/api/loans/99999")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_delete_loan_success() {
        let app = test::init_service(
            App::new()
                .route("/api/loans/{id}", web::delete().to(crate::controllers::loan_controller::delete_loan))
        ).await;

        let req = test::TestRequest::delete()
            .uri("/api/loans/1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }

    #[actix_rt::test]
    async fn test_delete_loan_not_found() {
        let app = test::init_service(
            App::new()
                .route("/api/loans/{id}", web::delete().to(crate::controllers::loan_controller::delete_loan))
        ).await;

        let req = test::TestRequest::delete()
            .uri("/api/loans/99999")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}

mod unit_tests {
    use super::*;
    use crate::models::loan::{Loan, LoanStatus};
    use crate::dto::loan_dto::{LoanRequest, LoanResponse};
    use chrono::NaiveDate;

    #[test]
    fn test_loan_creation() {
        let loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        assert_eq!(loan.is_active(), true);
    }

    #[test]
    fn test_loan_calculation_total_interest() {
        let loan = Loan::new(1, 10000.0, 10.0, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let interest = loan.calculate_total_interest();
        assert!(interest > 0.0);
    }

    #[test]
    fn test_loan_calculation_total_amount() {
        let loan = Loan::new(1, 10000.0, 10.0, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let total = loan.calculate_total_amount();
        assert!(total > 10000.0);
    }

    #[test]
    fn test_loan_update_amount() {
        let mut loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let result = loan.update_amount(15000.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_loan_update_invalid_amount() {
        let mut loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let result = loan.update_amount(-1000.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_loan_update_interest_rate() {
        let mut loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let result = loan.update_interest_rate(7.5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_loan_update_invalid_interest_rate() {
        let mut loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let result = loan.update_interest_rate(-2.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_loan_mark_as_paid() {
        let mut loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        loan.mark_as_paid();
        assert!(!loan.is_active());
    }

    #[test]
    fn test_loan_mark_as_defaulted() {
        let mut loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        loan.mark_as_defaulted();
        assert!(!loan.is_active());
    }

    #[test]
    fn test_loan_validate_valid() {
        let loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let result = loan.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_loan_validate_invalid_amount() {
        let loan = Loan::new(1, -1000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let result = loan.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_loan_validate_invalid_interest_rate() {
        let loan = Loan::new(1, 10000.0, -5.0, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let result = loan.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_loan_dto_from_loan() {
        let loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let dto = LoanResponse::from_loan(&loan);
        assert_eq!(dto.id, 1);
        assert_eq!(dto.amount, 10000.0);
    }

    #[test]
    fn test_loan_dto_from_loan_without_calculations() {
        let loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let dto = LoanResponse::from_loan_without_calculations(&loan);
        assert_eq!(dto.id, 1);
        assert_eq!(dto.amount, 10000.0);
    }
}

mod error_handling_tests {
    use super::*;
    use crate::error::loan_error::LoanError;

    #[test]
    fn test_loan_error_not_found() {
        let error = LoanError::NotFound(1);
        assert!(matches!(error, LoanError::NotFound(_)));
    }

    #[test]
    fn test_loan_error_validation() {
        let error = LoanError::Validation("Invalid amount".to_string());
        assert!(matches!(error, LoanError::Validation(_)));
    }

    #[test]
    fn test_loan_error_internal() {
        let error = LoanError::Internal("Database error".to_string());
        assert!(matches!(error, LoanError::Internal(_)));
    }
}

mod pagination_tests {
    use super::*;
    use crate::dto::loan_dto::PaginationMetadata;

    #[test]
    fn test_pagination_metadata_first_page() {
        let metadata = PaginationMetadata::new(100, 1, 10);
        assert_eq!(metadata.get_page(), 1);
        assert_eq!(metadata.get_per_page(), 10);
        assert_eq!(metadata.get_offset(), 0);
    }

    #[test]
    fn test_pagination_metadata_second_page() {
        let metadata = PaginationMetadata::new(100, 2, 10);
        assert_eq!(metadata.get_page(), 2);
        assert_eq!(metadata.get_offset(), 10);
    }

    #[test]
    fn test_pagination_metadata_empty() {
        let metadata = PaginationMetadata::empty(1, 10);
        assert_eq!(metadata.get_page(), 1);
        assert_eq!(metadata.get_per_page(), 10);
    }
}

mod validation_tests {
    use super::*;
    use validator::Validate;
    use crate::dto::loan_dto::LoanRequest;
    use chrono::NaiveDate;

    #[test]
    fn test_loan_request_validate_positive_amount() {
        let req = LoanRequest {
            amount: 1000.0,
            interest_rate: 5.0,
            due_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_loan_request_validate_negative_amount() {
        let req = LoanRequest {
            amount: -1000.0,
            interest_rate: 5.0,
            due_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_loan_request_validate_negative_interest_rate() {
        let req = LoanRequest {
            amount: 1000.0,
            interest_rate: -5.0,
            due_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_loan_request_validate_zero_interest_rate() {
        let req = LoanRequest {
            amount: 1000.0,
            interest_rate: 0.0,
            due_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        };
        assert!(req.validate().is_ok());
    }
}