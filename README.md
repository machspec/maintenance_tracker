# Maintenance Tracker

Maintenance Tracker is an advanced to-do application that tracks the status of various items in a MySQL database. Values tracked include an item's category, title, status, repair cost, maintainer's note, and past updates.

The database structure is as follows:

```mermaid
classDiagram
    direction RL
    class Category {
      id: INT [PK]
      title: VARCHAR [30]
      removed: TINYINT
    }

    class Entry {
      id: INT [PK]
      item_id: INT
      cost: INT
      note: TEXT
      status: TINYINT
      visible: TINYINT
      removed: TINYINT
      date: DATETIME
    }

    class Item {
      id: INT [PK]
      title: VARCHAR [30]
      category_id: INT
    }

    Item "*" --> "1" Category : category_id
    Entry "*" --> "1" Item : item_id
```
