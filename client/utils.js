
const pubkeyCmp = (pubkey1, pubkey2) => {
  if (pubkey1 < pubkey2) {
    return pubkey1;
  }
  return pubkey2;
}

module.exports = {
  pubkeyCmp
};