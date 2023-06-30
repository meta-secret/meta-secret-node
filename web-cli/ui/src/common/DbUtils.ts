import {openDB} from "idb";

async function openDb(dbName: string) {
  const db = await openDB(dbName, 1, {
    upgrade(db) {
      let storeNames = ["meta_passwords", "commit_log"];

      for (let storeName of storeNames) {
        db.createObjectStore(storeName);
      }
    },
  });
  return db;
}

window.idbGet = async function (dbName: string, storeName: string, key: string): Promise<any> {
  const db = await openDb(dbName);

  const tx = db.transaction(storeName, 'readwrite');
  const store = tx.objectStore(storeName);

  const entity = await store.get(key);

  await tx.done;
  
  return Promise.resolve(entity);
}

window.idbSave = async function (dbName: string, storeName: string, key: string, value: any): Promise<void> {
  console.log("Save to db. Key: ", JSON.stringify(key, null, 2));
  
  const db = await openDb(dbName);
  const tx = db.transaction(storeName, 'readwrite');
  const store = tx.objectStore(storeName);

  await store.put(value, key);

  await tx.done;
  return Promise.resolve();
}

window.idbFindAll = async function (dbName: string, storeName: string): Promise<any[]> {
  const db = await openDb(dbName);
  
  const tx = db.transaction(storeName, 'readwrite');
  const store = tx.objectStore(storeName);
  
  let cursor = await store.openCursor();
  
  let events: any[] = [];
  
  while (cursor) {
    events.push(cursor.value);
    cursor = await cursor.continue();
  }
  
  await tx.done;
  return Promise.resolve(events);
}