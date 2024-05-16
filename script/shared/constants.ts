import path from 'path';

export const defaultTmpDir = path.join(process.cwd(), './tmp');
export const defaultContextPath = path.join(process.cwd(), './context');
export const defaultArtifactPath = path.join(process.cwd(), './artifacts');

export const contractNames = [
  'hpl_mailbox',
  'hpl_validator_announce',
  'hpl_ism_aggregate',
  'hpl_ism_multisig',
  'hpl_ism_pausable',
  'hpl_ism_routing',
  'hpl_igp',
  'hpl_hook_aggregate',
  'hpl_hook_fee',
  'hpl_hook_merkle',
  'hpl_hook_pausable',
  'hpl_hook_routing',
  'hpl_hook_routing_custom',
  'hpl_hook_routing_fallback',
  'hpl_test_mock_hook',
  'hpl_test_mock_ism',
  'hpl_test_mock_msg_receiver',
  'hpl_igp_oracle',
  'hpl_warp_cw20',
  'hpl_warp_native',
];

export const REMOTE_REPO_NAME = 'many-things/cw-hyperlane';
export const REMOTE_MIN_VERSION = 'v0.0.6-rc8';

export const RELEASE_API_URL = `https://api.github.com/repos/${REMOTE_REPO_NAME}/releases`;
export const RELEASE_ARTIFACT_URL = (tag: string) =>
  `https://github.com/${REMOTE_REPO_NAME}/releases/download/${tag}/cw-hyperlane-${tag}.zip`;

export const DEFAULT_CRADLE_RPC_BASE_URL =
  'https://cradle-manager.ec1-prod.newmetric.xyz/cradle/proxy/{session_id}';
export const DEFAULT_CRADLE_REST_BASE_URL =
  'https://cradle-manager.ec1-prod.newmetric.xyz/cradle/proxy/{session_id}';
export const DEFAULT_CRADLE_GRPC_BASE_URL =
  '{session_id}.cradle-grpc.ec1-prod.newmetric.xyz:80';
