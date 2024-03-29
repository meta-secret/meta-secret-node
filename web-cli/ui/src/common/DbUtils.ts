import {openDB} from 'idb';

async function openDb(dbName: string) {
  const db = await openDB(dbName, 1, {
    upgrade(db) {
      let storeNames = ["meta_vault", "user_credentials", "meta_passwords"];

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
  const db = await openDb(dbName);
  const tx = db.transaction(storeName, 'readwrite');
  const store = tx.objectStore(storeName);

  await store.put(value, key);

  await tx.done;
  return Promise.resolve();
}
