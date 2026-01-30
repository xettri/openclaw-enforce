use anyhow::Result;
use std::net::SocketAddr;
use std::path::Path;
use tonic::{transport::Server, Request, Response, Status};
use tracing::{error, info, warn};

use crate::fs::validator::PathValidator;
use crate::policy::Policy;
use crate::proto::openclaw::enforce::*;

/// Enforcement service implementation
pub struct EnforcementServiceImpl {
    validator: PathValidator,
}

impl EnforcementServiceImpl {
    pub fn new(policy: Policy) -> Self {
        let validator = PathValidator::new(policy.filesystem.clone());
        Self { validator }
    }

    fn create_security_status(&self, allowed: bool, reason: String, violations: Vec<String>) -> SecurityStatus {
        SecurityStatus {
            allowed,
            reason,
            violations,
        }
    }
}

#[tonic::async_trait]
impl enforcement_service_server::EnforcementService for EnforcementServiceImpl {
    async fn read_file(
        &self,
        request: Request<ReadFileRequest>,
    ) -> Result<Response<ReadFileResponse>, Status> {
        let req = request.into_inner();
        info!("ReadFile request: path={}", req.path);

        let path = Path::new(&req.path);

        // Validate path against policy
        match self.validator.can_read(path) {
            Ok(true) => {
                // Attempt to read the file
                match std::fs::read(&req.path) {
                    Ok(data) => {
                        info!("✅ File read successful: {} ({} bytes)", req.path, data.len());
                        Ok(Response::new(ReadFileResponse {
                            data,
                            status: Some(self.create_security_status(
                                true,
                                "Access granted".to_string(),
                                vec![],
                            )),
                        }))
                    }
                    Err(e) => {
                        error!("File system error reading {}: {}", req.path, e);
                        Err(Status::not_found(format!("File not found: {}", e)))
                    }
                }
            }
            Ok(false) => {
                warn!("❌ Access denied: {} (not in allowed paths)", req.path);
                Ok(Response::new(ReadFileResponse {
                    data: vec![],
                    status: Some(self.create_security_status(
                        false,
                        "Path not in allowed read list".to_string(),
                        vec!["path_not_allowed".to_string()],
                    )),
                }))
            }
            Err(e) => {
                error!("Path validation error for {}: {}", req.path, e);
                Err(Status::invalid_argument(format!(
                    "Path validation failed: {}",
                    e
                )))
            }
        }
    }

    async fn write_file(
        &self,
        _request: Request<WriteFileRequest>,
    ) -> Result<Response<WriteFileResponse>, Status> {
        Err(Status::unimplemented("write_file not yet implemented"))
    }

    async fn list_directory(
        &self,
        _request: Request<ListDirectoryRequest>,
    ) -> Result<Response<ListDirectoryResponse>, Status> {
        Err(Status::unimplemented("list_directory not yet implemented"))
    }

    async fn delete_file(
        &self,
        _request: Request<DeleteFileRequest>,
    ) -> Result<Response<DeleteFileResponse>, Status> {
        Err(Status::unimplemented("delete_file not yet implemented"))
    }

    async fn http_request(
        &self,
        _request: Request<HttpRequestData>,
    ) -> Result<Response<HttpResponseData>, Status> {
        Err(Status::unimplemented("http_request not yet implemented"))
    }

    async fn dns_lookup(
        &self,
        _request: Request<DnsLookupRequest>,
    ) -> Result<Response<DnsLookupResponse>, Status> {
        Err(Status::unimplemented("dns_lookup not yet implemented"))
    }

    async fn execute_command(
        &self,
        _request: Request<ExecuteCommandRequest>,
    ) -> Result<Response<ExecuteCommandResponse>, Status> {
        Err(Status::unimplemented("execute_command not yet implemented"))
    }

    async fn request_capability(
        &self,
        _request: Request<CapabilityRequest>,
    ) -> Result<Response<CapabilityResponse>, Status> {
        Err(Status::unimplemented("request_capability not yet implemented"))
    }

    async fn revoke_capability(
        &self,
        _request: Request<RevokeRequest>,
    ) -> Result<Response<RevokeResponse>, Status> {
        Err(Status::unimplemented("revoke_capability not yet implemented"))
    }

    async fn get_status(
        &self,
        _request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        info!("Status request received");
        
        Ok(Response::new(StatusResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
            healthy: true,
            active_policy: Some(PolicyInfo {
                path: "policy.toml".to_string(),
                loaded_at: 0,
            }),
            resources: Some(ResourceUsage {
                memory_bytes: 0,
                cpu_percent: 0.0,
                active_connections: 0,
            }),
        }))
    }

    type GetAuditLogsStream = tokio_stream::wrappers::ReceiverStream<Result<AuditLogEntry, Status>>;

    async fn get_audit_logs(
        &self,
        _request: Request<AuditLogRequest>,
    ) -> Result<Response<Self::GetAuditLogsStream>, Status> {
        Err(Status::unimplemented("get_audit_logs not yet implemented"))
    }
}

pub async fn serve(addr: SocketAddr, policy: Policy) -> Result<()> {
    info!("Starting OpenClaw Enforce gRPC server");

    // Health service
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<tonic_health::pb::health_server::HealthServer<
            tonic_health::server::HealthReporter,
        >>()
        .await;

    // Enforcement service
    let enforcement_service = EnforcementServiceImpl::new(policy);
    
    info!("Services registered:");
    info!("  - grpc.health.v1.Health");
    info!("  - openclaw.enforce.EnforcementService");
    info!("gRPC server listening on {}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(enforcement_service_server::EnforcementServiceServer::new(
            enforcement_service,
        ))
        .serve(addr)
        .await?;

    Ok(())
}
