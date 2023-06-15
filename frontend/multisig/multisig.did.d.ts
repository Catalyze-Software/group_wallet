import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Result = { 'Ok' : string } |
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
  'get_whitelist' : ActorMethod<[], Array<Principal>>,
  'get_whitelist_requests' : ActorMethod<
    [[] | [Status]],
    Array<WhitelistRequestData>
  >,
  'vote_on_whitelist_request' : ActorMethod<[number, VoteType], Result>,
  'whitelist_request' : ActorMethod<[WhitelistRequestType], Result>,
}
