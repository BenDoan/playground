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
    val num_barracks = rc.getTeamMemory()(comm_types.indexOf(RobotType.BARRACKS))

    if (rc.isWeaponReady()){
      attack_closest(rc)
    }

    if (rc.isCoreReady()) {
      if (rc.getTeamOre() < 300 || num_barracks >= MAX_BARRACKS) {
        rc.mine()
      }

      if (rc.getTeamOre() >= 300 && num_barracks < MAX_BARRACKS) {
        rc.build(Direction.NORTH, RobotType.BARRACKS)
        rc.getTeamMemory()(comm_types.indexOf(RobotType.BARRACKS)) += 1
      }

      val d = directions(r.nextInt(8))
      if (rc.canMove(d)) {
        rc.move(d)
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

}
