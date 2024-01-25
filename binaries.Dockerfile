# Temporary, will be flashbots once tested
FROM ruteri/gramine-andromeda-revm:latest as builder

FROM scratch as binaries

COPY --from=builder /sgx-revm.sig /
COPY --from=builder /sgx-revm.manifest /
COPY --from=builder /sgx-revm.manifest.sgx /
COPY --from=builder /target/release/gramine-sirrah /target/release/
