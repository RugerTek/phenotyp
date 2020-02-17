mod Waypoint;

struct Route {
    start: Waypoint::Waypoint;
    finish: Waypoint;

    waypoints: [Waypoint];
}

