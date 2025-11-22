export function useTransaction() {
  async function send(tx: any) {
    // TODO: integrar com carteira e provider Anchor
    return tx;
  }

  return { send };
}
