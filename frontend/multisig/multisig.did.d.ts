import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export interface Dip20TransferArgs { 'to' : Principal, 'amount' : bigint }
export type Result = { 'Ok' : null } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : string } |
  { 'Err' : string };
export interface SharedData {
  'id' : number,
  'status' : Status,
  'votes' : Votes,
  'created_at' : bigint,
  'requested_by' : Principal,
}
export type Status = { 'Deadlock' : null } |
  { 'Approved' : null } |
  { 'Rejected' : null } |
  { 'Expired' : null } |
  { 'Pending' : null };
export type TokenStandard = { 'ICRC1' : null } |
  { 'DIP20' : null };
export interface TransactionRequestData {
  'args' : TransferRequestType,
  'data' : SharedData,
  'canister_id' : Principal,
}
export interface TransferArgs {
  'to' : Account,
  'fee' : [] | [bigint],
  'memo' : [] | [Uint8Array | number[]],
  'from_subaccount' : [] | [Uint8Array | number[]],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
}
export type TransferRequestType = { 'ICRC1' : TransferArgs } |
  { 'DIP20' : Dip20TransferArgs };
export type VoteType = { 'Approve' : null } |
  { 'Reject' : null };
export interface Votes {
  'rejections' : Array<Principal>,
  'approvals' : Array<Principal>,
}
export interface WhitelistRequestData {
  'request_type' : WhitelistRequestType,
  'data' : SharedData,
}
export type WhitelistRequestType = { 'Add' : Principal } |
  { 'Remove' : Principal };
export interface _SERVICE {
  '__get_candid_interface_tmp_hack' : ActorMethod<[], string>,
  'add_token_from_list' : ActorMethod<[Principal, TokenStandard], Result>,
  'get_time_out' : ActorMethod<[], bigint>,
  'get_token_list' : ActorMethod<[], Array<[Principal, TokenStandard]>>,
  'get_transaction_requests' : ActorMethod<
    [[] | [Status]],
    Array<TransactionRequestData>
  >,
  'get_whitelist' : ActorMethod<[], Array<Principal>>,
  'get_whitelist_requests' : ActorMethod<
    [[] | [Status]],
    Array<WhitelistRequestData>
  >,
  'remove_token_from_list' : ActorMethod<[Principal], Result>,
  'transaction_request' : ActorMethod<
    [Principal, TransferRequestType],
    Result_1
  >,
  'vote_on_transaction_request' : ActorMethod<[number, VoteType], Result_1>,
  'vote_on_whitelist_request' : ActorMethod<[number, VoteType], Result_1>,
  'whitelist_request' : ActorMethod<[WhitelistRequestType], Result_1>,
}
