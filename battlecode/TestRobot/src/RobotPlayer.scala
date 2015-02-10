package scalaplayer

import battlecode.common._

object RobotPlayer {
    val r = scala.util.Random
    val directions = List(Direction.NORTH, Direction.NORTH_EAST, Direction.EAST, Direction.SOUTH_EAST, Direction.SOUTH, Direction.SOUTH_WEST, Direction.WEST, Direction.NORTH_WEST);
    val comm_types = List(RobotType.BEAVER, RobotType.SOLDIER, RobotType.BARRACKS)

    val MAX_BEAVERS = 5;
    val MAX_BARRACKS = 5;
    val MAX_SOLDIERS = 30;
    val MAX_BASHERS = 40;

    def run(rc: RobotController) {
        while(true) {
            try {
              rc.getType() match {
                case RobotType.HQ => run_hq(rc)
                case RobotType.BEAVER => run_beaver(rc)
                case RobotType.BARRACKS => run_barracks(rc)
                case _ => 1
              }
            } catch {
                case e : Exception => {
                    println("caught exception:")
                    e.printStackTrace()
                }
            }
        }
    }

  def run_hq(rc: RobotController): Unit ={
    val t = RobotType.BEAVER

    if (rc.getTeamMemory()(comm_types.indexOf(t)) < MAX_BEAVERS) {
      rc.build(Direction.NORTH, t);
      rc.getTeamMemory()(comm_types.indexOf(t)) += 1
    }
  }

  def run_beaver(rc: RobotController): Unit ={
      if (rc.isWeaponReady()) {
        attack_closest(rc)
      }
      if (rc.isCoreReady()) {
        val fate = r.nextInt(1000);
        if (fate < 8 && rc.getTeamOre() >= 300) {
          tryBuild(rc, directions(r.nextInt(8)),RobotType.BARRACKS);
        } else if (fate < 600) {
          rc.mine();
        } else if (fate < 900) {
          tryMove(rc, directions(r.nextInt(8)));
        } else {
          tryMove(rc, rc.senseHQLocation().directionTo(rc.getLocation()));
        }
      }
  }

  def run_tower(rc: RobotController): Unit ={
    if (rc.isWeaponReady()){
      attack_closest(rc)
    }
  }

  def run_barracks(rc: RobotController): Unit ={
    if (rc.getTeamOre() > 80 && rc.getTeamMemory()(comm_types.indexOf(RobotType.SOLDIER)) < MAX_SOLDIERS){
      rc.build(Direction.NORTH, RobotType.SOLDIER)
    }else if (rc.getTeamOre() > 80 && rc.getTeamMemory()(comm_types.indexOf(RobotType.BASHER)) < MAX_BASHERS){
      rc.build(Direction.SOUTH, RobotType.BASHER)
    }
  }

  def attack_closest(rc: RobotController): Unit ={
    val enemies = rc.senseNearbyRobots(RobotType.TOWER.attackRadiusSquared)
    if (enemies.length > 0){
      rc.attackLocation(enemies(0).location)
    }
  }

  def tryBuild(rc: RobotController, d: Direction, t: RobotType){
    var offsetIndex = 0;
    val offsets = List(0,1,-1,2,-2,3,-3,4)
    val dirint = directionToInt(d);

    offsets(offsetIndex) + 1
    while (offsetIndex < 8 && !rc.canMove(directions(dirint + offsets(offsetIndex) + 8 % 8))) {
      offsetIndex += 1;
    }
    if (offsetIndex < 8) {
      rc.build(directions((dirint + offsets(offsetIndex) + 8) % 8), t)
    }
  }

  def tryMove(rc: RobotController, d: Direction) {
    var offsetIndex = 0;
    val offsets = List(0,1,-1,2,-2,3,-3,4)
    val dirint = directionToInt(d);

    while (offsetIndex < 5 && !rc.canMove(directions((dirint + offsets(offsetIndex) + 8) % 8))) {
      offsetIndex += 1;
    }
    if (offsetIndex < 5) {
      rc.move(directions((dirint+offsets(offsetIndex)+8)%8));
    }
  }


  def directionToInt(d: Direction): Int ={
    d match {
      case Direction.NORTH => 0
      case Direction.NORTH_EAST => 1
      case Direction.EAST => 2
      case Direction.SOUTH_EAST => 3
      case Direction.SOUTH => 4
      case Direction.SOUTH_WEST => 5
      case Direction.WEST => 6
      case Direction.NORTH_WEST => 7
      case _ => -1
    }
  }


}
