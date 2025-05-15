export async function fetchAllItems() {
  const proxy = "https://corsproxy.io/?";
  const url = "https://api.wynncraft.com/v2/item/search";

  const res = await fetch(proxy + encodeURIComponent(url));
  const json = await res.json();
  const items = json.data;

  const categorized = {
    helmets: [],
    chestplates: [],
    leggings: [],
    boots: [],
    rings: [],
    bracelets: [],
    necklaces: [],
    weapons: []
  };

  items.forEach(item => {
    switch ((item.type || "").toLowerCase()) {
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
  });

  return categorized;
}
