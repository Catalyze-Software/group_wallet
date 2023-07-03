export const idlFactory = ({ IDL }) => {
  const TokenStandard = IDL.Variant({ 'ICRC1' : IDL.Null, 'DIP20' : IDL.Null });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const Status = IDL.Variant({
    'Deadlock' : IDL.Null,
    'Approved' : IDL.Null,
    'Rejected' : IDL.Null,
    'Expired' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const TransferArgs = IDL.Record({
    'to' : Account,
    'fee' : IDL.Opt(IDL.Nat),
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'from_subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'created_at_time' : IDL.Opt(IDL.Nat64),
    'amount' : IDL.Nat,
  });
  const Dip20TransferArgs = IDL.Record({
    'to' : IDL.Principal,
    'amount' : IDL.Nat64,
  });
  const TransferRequestType = IDL.Variant({
    'ICRC1' : TransferArgs,
    'DIP20' : Dip20TransferArgs,
  });
  const Votes = IDL.Record({
    'rejections' : IDL.Vec(IDL.Principal),
    'approvals' : IDL.Vec(IDL.Principal),
  });
  const SharedData = IDL.Record({
    'id' : IDL.Nat32,
    'status' : Status,
    'votes' : Votes,
    'created_at' : IDL.Nat64,
    'requested_by' : IDL.Principal,
  });
  const TransactionRequestData = IDL.Record({
    'args' : TransferRequestType,
    'data' : SharedData,
    'canister_id' : IDL.Principal,
  });
  const WhitelistRequestType = IDL.Variant({
    'Add' : IDL.Principal,
    'Remove' : IDL.Principal,
  });
  const WhitelistRequestData = IDL.Record({
    'request_type' : WhitelistRequestType,
    'data' : SharedData,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  const VoteType = IDL.Variant({ 'Approve' : IDL.Null, 'Reject' : IDL.Null });
  return IDL.Service({
    '__get_candid_interface_tmp_hack' : IDL.Func([], [IDL.Text], ['query']),
    'add_token_from_list' : IDL.Func(
        [IDL.Principal, TokenStandard],
        [Result],
        [],
      ),
    'get_time_out' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_token_list' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, TokenStandard))],
        ['query'],
      ),
    'get_transaction_requests' : IDL.Func(
        [IDL.Opt(Status)],
        [IDL.Vec(TransactionRequestData)],
        ['query'],
      ),
    'get_whitelist' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'get_whitelist_requests' : IDL.Func(
        [IDL.Opt(Status)],
        [IDL.Vec(WhitelistRequestData)],
        ['query'],
      ),
    'remove_token_from_list' : IDL.Func([IDL.Principal], [Result], []),
    'transaction_request' : IDL.Func(
        [IDL.Principal, TransferRequestType],
        [Result_1],
        [],
      ),
    'vote_on_transaction_request' : IDL.Func(
        [IDL.Nat32, VoteType],
        [Result_1],
        [],
      ),
    'vote_on_whitelist_request' : IDL.Func(
        [IDL.Nat32, VoteType],
        [Result_1],
        [],
      ),
    'whitelist_request' : IDL.Func([WhitelistRequestType], [Result_1], []),
  });
};
export const init = ({ IDL }) => { return [IDL.Principal]; };
