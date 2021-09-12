// ECS ~ entity component system
// entitites are logicless, atomic units, merely more than IDs
// components are properties ordered to entities adding/describing their properties, although logicless
// systems provide one elemnt of simulation, e.g. render; they provide the game logic 
// resources are shared data to multiple systems
// composing entities = combining components


// Systems
// queries the ECS for data and performs operations
// runs concurrently and is managed by Scheduler
// they shouldn't see the innards of each-other> they are ideal for module-based organization

