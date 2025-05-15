export async function fetchAllItems() {
  const res = await fetch("https://corsproxy.io/?" + encodeURIComponent("https://api.wynncraft.com/v2/item/search"));
  const json = await res.json();
  const items = json.data;

  const categorized = {
    helmets: [] as any[],
    chestplates: [],
    leggings: [],
    boots: [],
    rings: [],
    bracelets: [],
    necklaces: [],
    weapons: []
  };

  for (const item of items) {
    const type = item.type?.toLowerCase();
    switch (type) {
      case "helmet": categorized.helmets.push(item); break;
      case "chestplate": categorized.chestplates.push(item); break;
      case "leggings": categorized.leggings.push(item); break;
      case "boots": categorized.boots.push(item); break;
      case "ring": categorized.rings.push(item); break;
      case "bracelet": categorized.bracelets.push(item); break;
      case "necklace": categorized.necklaces.push(item); break;
      case "wand":
      case "bow":
      case "spear":
      case "dagger":
      case "relik": categorized.weapons.push(item); break;
    }
  }

  return categorized;
}
