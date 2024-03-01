import path from "path";

export const defaultTmpDir = path.join(process.cwd(), "./tmp");
export const defaultContextPath = path.join(process.cwd(), "./context");
export const defaultArtifactPath = path.join(process.cwd(), "./artifacts");

export const contractNames = [
  "hpl_mailbox",
  "hpl_validator_announce",
  "hpl_ism_aggregate",
  "hpl_ism_multisig",
  "hpl_ism_pausable",
  "hpl_ism_routing",
  "hpl_igp",
  "hpl_hook_aggregate",
  "hpl_hook_fee",
  "hpl_hook_merkle",
  "hpl_hook_pausable",
  "hpl_hook_routing",
  "hpl_hook_routing_custom",
  "hpl_hook_routing_fallback",
  "hpl_test_mock_hook",
  "hpl_test_mock_ism",
  "hpl_test_mock_msg_receiver",
  "hpl_igp_oracle",
  "hpl_warp_cw20",
  "hpl_warp_native",
];
