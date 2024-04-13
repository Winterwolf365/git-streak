reload(); // initial load

setInterval(set_streak, 5000); // watch for commits (streak change)

async function set_streak() {
  document.querySelector("#streak").textContent = "Str>eak " /* add > for underline in special font */ + await invoke("get_streak");
}

async function set_repositories() {
  let repositories_div = document.querySelector("#repositories");
  let repositories = await invoke("get_repositories");

  if (repositories.length == 0) {
    repositories_div.innerHTML = "<p>No repositories in database.</p>";
  } else {
    repositories_div.innerHTML = "";

    repositories.forEach(repository => {
      repositories_div.innerHTML += `<main><p>${repository[1]}</p><button onclick="delete_repository(${repository[0]})">delete</button></main>`;
    });
  }
}

function reload() {
  set_streak();
  set_repositories();
}

async function add_repositories() {
  await invoke("add_repositories").then((message) => {
  }).catch((repository) => {
    document.querySelector("#error").innerHTML = `${repository} <button onclick="remove_error()" class="remove-error">remove</button>`;
    document.querySelector("#error").classList.add("error");
  });
  reload();
}

async function delete_repository(id) {
  await invoke('delete_repository', { id: id });
  reload();
}

function remove_error() {
  document.querySelector("#error").textContent = "";
  document.querySelector("#error").classList.remove("error");
}