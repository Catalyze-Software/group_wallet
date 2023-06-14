export const idlFactory = ({ IDL }) => {
  const VoteType = IDL.Variant({
    'Rejection' : IDL.Null,
    'Approval' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  const WhitelistRequestType = IDL.Variant({
    'Add' : IDL.Principal,
    'Remove' : IDL.Principal,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  return IDL.Service({
    '__get_candid_interface_tmp_hack' : IDL.Func([], [IDL.Text], ['query']),
    'get_whitelist' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'vote_on_whitelist_request' : IDL.Func([IDL.Nat32, VoteType], [Result], []),
    'whitelist_request' : IDL.Func([WhitelistRequestType], [Result_1], []),
  });
};
export const init = ({ IDL }) => { return [IDL.Principal]; };
