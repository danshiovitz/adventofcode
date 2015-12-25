using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;

public class prog
{
  private static readonly int HERO_HITPOINTS = 100;

  static private void Run(string bossFilename, string equipFilename) {
    Creature boss = LoadCreature("Boss", bossFilename);
    List<Equipment> equipment = LoadEquipment(equipFilename);
    var heros = GenerateHeros(HERO_HITPOINTS, equipment);
    var winningHeros = heros.Where(p => p.Defeats(boss)).OrderBy(p => p.Cost);
    var losingHeros = heros.Where(p => !p.Defeats(boss)).OrderBy(p => p.Cost);
    Console.WriteLine("Cheapest winning Hero: {0}", winningHeros.First());
    Console.WriteLine("Costliest losing Hero: {0}", losingHeros.Last());
  }

  private static readonly Regex attrRegex = new Regex(@"(.*): (.*)");

  static private Creature LoadCreature(string creatureName, string filename) {
    Creature creature = new Creature();
    creature.Name = creatureName;

    foreach (string line in File.ReadLines(filename)) {
      Match match = attrRegex.Match(line);

      int value;
      if (!match.Success || !Int32.TryParse(match.Groups[2].Value, out value)) {
        throw new Exception(String.Format("Bad line: {0}", line));
      }

      string name = match.Groups[1].Value;
      if (name.Equals("Hit Points")) {
        creature.HitPoints = value;
      } else if (name.Equals("Damage")) {
        creature.Damage = value;
      } else if (name.Equals("Armor")) {
        creature.Armor = value;
      }
    }

    return creature;
  }

  private static readonly Regex catRegex = new Regex(@"(.*):\s+Cost\s+Damage\s+Armor");
  private static readonly Regex itemRegex = new Regex(@"^(.*?)\s+(\d+)\s+(\d+)\s+(\d+)");
  static private List<Equipment> LoadEquipment(String filename) {
    List<Equipment> items = new List<Equipment>();

    string category = null;
    foreach (string line in File.ReadLines(filename)) {
      if (line.Equals("")) {
        category = null;
        continue;
      }

      Match asCat = catRegex.Match(line);
      if (asCat.Success) {
        category = asCat.Groups[1].Value;
        continue;
      }

      Match asItem = itemRegex.Match(line);
      Equipment item = new Equipment();

      int cost, damage, armor;
      if (!asItem.Success ||
          !Int32.TryParse(asItem.Groups[2].Value, out cost) ||
          !Int32.TryParse(asItem.Groups[3].Value, out damage) ||
          !Int32.TryParse(asItem.Groups[4].Value, out armor)) {
        throw new Exception(String.Format("Bad line: {0}", line));
      }

      if (category == null) {
        throw new Exception("Category not set?");
      }

      item.Name = asItem.Groups[1].Value;
      if (category.Equals("Rings")) {
        item.Name = "Ring of " + item.Name;
      }
      item.Category = category;
      item.Cost = cost;
      item.Damage = damage;
      item.Armor = armor;
      items.Add(item);
    }

    return items;
  }

  static private IEnumerable<Creature> GenerateHeros(int hitpoints, List<Equipment> equipment) {
    List<Equipment> weapons = equipment.Where(a => a.Category.Equals("Weapons")).ToList();
    List<Equipment> armor = equipment.Where(a => a.Category.Equals("Armor")).ToList();
    List<Equipment> rings = equipment.Where(a => a.Category.Equals("Rings")).ToList();

    for (int w = 0; w < weapons.Count; w++) {
      for (int a = 0; a < armor.Count + 1; a++) {
        for (int r1 = 0; r1 < rings.Count + 1; r1++) {
          for (int r2 = Math.Min(r1 + 1, rings.Count); r2 < rings.Count + 1; r2++) {
            List<Equipment> inventory = new List<Equipment>();
            if (w < weapons.Count) {
              inventory.Add(weapons[w]);
            }
            if (a < armor.Count) {
              inventory.Add(armor[a]);
            }
            if (r1 < rings.Count) {
              inventory.Add(rings[r1]);
            }
            if (r2 < rings.Count) {
              inventory.Add(rings[r2]);
            }

            Creature hero = new Creature();
            hero.Name = "Hero";
            hero.HitPoints = hitpoints;
            hero.Stuff = inventory;
            hero.Damage = inventory.Select(x => x.Damage).Sum();
            hero.Armor = inventory.Select(x => x.Armor).Sum();

            //Console.WriteLine(String.Format("A hero arises: {0}", hero));
            yield return hero;
          }
        }
      }
    }
  }

  static public void Main(string[] args) {
    Run(args[0], args[1]);
  }

  private class Creature {
    public string Name { get; set; }
    public int HitPoints { get; set; }
    public int Damage { get; set; }
    public int Armor { get; set; }
    public int Cost { get { return Stuff.Select(a => a.Cost).Sum(); } }
    public List<Equipment> Stuff { get; set; }

    public Creature() {
      this.Stuff = new List<Equipment>();
    }

    override public String ToString() {
      return String.Format("{0}: {1} hp, {2} damage, {3} armor, {4} cost ({5})",
                           this.Name, this.HitPoints, this.Damage, this.Armor,
                           this.Cost, String.Join(", ", this.Stuff.Select(a => a.Name)));
    }

    private int Attack(Creature other) {
      return Math.Max((this.Damage - other.Armor), 1);
    }

    public bool Defeats(Creature other) {
      int thisHp = this.HitPoints;
      int otherHp = other.HitPoints;

      while (thisHp > 0 && otherHp > 0) {
        otherHp -= this.Attack(other);

        if (otherHp <= 0) {
          break;
        }

        thisHp -= other.Attack(this);
      }

      return thisHp > 0;
    }
  }

  private class Equipment {
    public string Name { get; set; }
    public int Damage { get; set; }
    public int Armor { get; set; }
    public int Cost { get; set; }
    public string Category { get; set; }
  }
}
