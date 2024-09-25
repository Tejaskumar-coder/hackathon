#![allow(non_snake_case)]
#![no_std]

use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short, Map};

// Struct to store job listing details
#[contracttype]
#[derive(Clone)]
pub struct JobListing {
    pub job_id: u64,
    pub title: String,      // Job title
    pub description: String, // Job description
    pub employer: String,   // Employer's identifier
    pub application_fee: u64, // Application fee in Lumens (XLM)
    pub is_active: bool,    // Is the job listing still active?
}

// Struct to store job application details
#[contracttype]
#[derive(Clone)]
pub struct JobApplication {
    pub application_id: u64,
    pub job_id: u64,      // Job ID to which this application applies
    pub applicant: String, // Applicant's identifier
    pub status: String,    // Application status (Pending, Interviewed, Hired, Rejected)
}

// Constants for project and transaction identifiers
const JOB_LISTING_COUNT: Symbol = symbol_short!("J_LST_CNT");
const APPLICATION_COUNT: Symbol = symbol_short!("A_LST_CNT");

#[contract]
pub struct TalentRecruitmentContract;

#[contractimpl]
impl TalentRecruitmentContract {

    // This function allows employers to post a job
    pub fn post_job(env: Env, title: String, description: String, employer: String, application_fee: u64) -> u64 {
        let mut job_count: u64 = env.storage().instance().get(&JOB_LISTING_COUNT).unwrap_or(0);
        job_count += 1;

        let job_listing = JobListing {
            job_id: job_count,
            title: title.clone(),
            description: description.clone(),
            employer: employer.clone(),
            application_fee: application_fee,
            is_active: true,
        };

        let mut job_map: Map<u64, JobListing> = env.storage().instance().get(&Symbol::short("JOB_LISTINGS")).unwrap_or(Map::new(&env));
        job_map.set(job_count, job_listing.clone());

        env.storage().instance().set(&Symbol::short("JOB_LISTINGS"), &job_map);
        env.storage().instance().set(&JOB_LISTING_COUNT, &job_count);

        log!(&env, "Job Posted: {} by Employer: {} with Job ID: {}", title, employer, job_count);
        job_count
    }

    // This function allows candidates to apply for a job by paying the application fee
    pub fn apply_for_job(env: Env, job_id: u64, applicant: String) -> u64 {
        let mut app_count: u64 = env.storage().instance().get(&APPLICATION_COUNT).unwrap_or(0);
        app_count += 1;

        // Retrieve job listing
        let mut job_map: Map<u64, JobListing> = env.storage().instance().get(&Symbol::short("JOB_LISTINGS")).unwrap_or(Map::new(&env));
        let job_listing = job_map.get(job_id).unwrap_or_else(|| {
            panic!("Job listing not found");
        });

        // Make sure job is active
        if !job_listing.is_active {
            panic!("This job listing is no longer active");
        }

        // Create job application
        let job_application = JobApplication {
            application_id: app_count,
            job_id: job_id,
            applicant: applicant.clone(),
            status: String::from_str(&env, "Pending"),
        };

        // Store application
        let mut app_map: Map<u64, JobApplication> = env.storage().instance().get(&Symbol::short("APPLICATIONS")).unwrap_or(Map::new(&env));
        app_map.set(app_count, job_application.clone());

        env.storage().instance().set(&Symbol::short("APPLICATIONS"), &app_map);
        env.storage().instance().set(&APPLICATION_COUNT, &app_count);

        log!(&env, "Application received from {} for Job ID: {}", applicant, job_id);
        app_count
    }

    // This function allows employers to update the application status (Interview, Hired, Rejected)
    pub fn update_application_status(env: Env, application_id: u64, status: String) {
        let mut app_map: Map<u64, JobApplication> = env.storage().instance().get(&Symbol::short("APPLICATIONS")).unwrap_or(Map::new(&env));
        let mut application = app_map.get(application_id).unwrap_or_else(|| {
            panic!("Application not found");
        });

        application.status = status.clone();
        app_map.set(application_id, application);

        env.storage().instance().set(&Symbol::short("APPLICATIONS"), &app_map);
        log!(&env, "Application ID: {} updated to status: {}", application_id, status);
    }

    // This function allows employers to close the job listing
    pub fn close_job(env: Env, job_id: u64) {
        let mut job_map: Map<u64, JobListing> = env.storage().instance().get(&Symbol::short("JOB_LISTINGS")).unwrap_or(Map::new(&env));
        let mut job_listing = job_map.get(job_id).unwrap_or_else(|| {
            panic!("Job listing not found");
        });

        job_listing.is_active = false;
        job_map.set(job_id, job_listing.clone());

        env.storage().instance().set(&Symbol::short("JOB_LISTINGS"), &job_map);
        log!(&env, "Job listing ID: {} has been closed", job_id);
    }

    // This function retrieves the details of a specific job listing
    pub fn view_job(env: Env, job_id: u64) -> JobListing {
        let job_map: Map<u64, JobListing> = env.storage().instance().get(&Symbol::short("JOB_LISTINGS")).unwrap_or(Map::new(&env));
        job_map.get(job_id).unwrap_or_else(|| {
            panic!("Job listing not found")
        })
    }

    // This function retrieves the details of a specific job application
    pub fn view_application(env: Env, application_id: u64) -> JobApplication {
        let app_map: Map<u64, JobApplication> = env.storage().instance().get(&Symbol::short("APPLICATIONS")).unwrap_or(Map::new(&env));
        app_map.get(application_id).unwrap_or_else(|| {
            panic!("Application not found")
        })
    }
}
