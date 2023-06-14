export const idlFactory = ({ IDL }) => {
  const Status = IDL.Variant({
    'Approved' : IDL.Null,
    'Rejected' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const WhitelistRequestType = IDL.Variant({
    'Add' : IDL.Principal,
    'Remove' : IDL.Principal,
  });
  const Votes = IDL.Record({
    'rejections' : IDL.Vec(IDL.Principal),
    'approvals' : IDL.Vec(IDL.Principal),
  });
  const SharedData = IDL.Record({
    'status' : Status,
    'votes' : Votes,
    'created_at' : IDL.Nat64,
    'requested_by' : IDL.Principal,
  });
  const WhitelistRequestData = IDL.Record({
    'request_type' : WhitelistRequestType,
    'data' : SharedData,
  });
  const VoteType = IDL.Variant({ 'Approve' : IDL.Null, 'Reject' : IDL.Null });
  const Result = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  return IDL.Service({
    '__get_candid_interface_tmp_hack' : IDL.Func([], [IDL.Text], ['query']),
    'get_whitelist' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'get_whitelist_requests' : IDL.Func(
        [IDL.Opt(Status)],
        [IDL.Vec(WhitelistRequestData)],
        ['query'],
      ),
    'vote_on_whitelist_request' : IDL.Func([IDL.Nat32, VoteType], [Result], []),
    'whitelist_request' : IDL.Func([WhitelistRequestType], [Result_1], []),
  });
};
export const init = ({ IDL }) => { return [IDL.Principal]; };
