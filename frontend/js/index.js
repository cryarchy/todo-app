// Full spec-compliant TodoMVC with database persistence
// and hash-based routing in ~120 effective lines of JavaScript.

// app Vue instance
var app = new Vue({
  // app initial state
  data: {
    todos: [],
    credentials: {
      username: "",
      password: "",
      cpassword: ""
    },
    remaining: 0,
    newTodo: "",
    editedTodo: null,
    visibility: "all",
    user: null
  },

  // watch todos change for localStorage persistence
  watch: {
    visibility(val) {
      if (["all", "active", "completed"].indexOf(val) !== -1) {
        this.fetchTodos(val);
      }
    }
  },

  // computed properties
  // http://vuejs.org/guide/computed.html
  computed: {
    filteredTodos: function() {
      return filters[this.visibility](this.todos);
    },
    remaining: function() {
      return filters.active(this.todos).length;
    },
    allDone: {
      get: function() {
        return this.remaining === 0;
      },
      set: function(value) {
        this.todos.forEach(function(todo) {
          todo.completed = value;
        });
      }
    }
  },

  filters: {
    pluralize: function(n) {
      return n === 1 ? "item" : "items";
    }
  },

  // methods that implement data logic.
  // note there's no DOM manipulation here at all.
  methods: {
    async postQuery(query, variables, ignoreError) {
      const resp = await fetch("/graphql", {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({ query, variables: variables || {} })
      });
      const { data, errors } = await resp.json();
      if (errors && !ignoreError) {
        alert(errors.map(({ message }) => message).join(","));
        return {};
      }
      return { data, errors };
    },

    async fetchTodos(show) {
      const query = `
        query($show: String!) {
            Todo {
                get(show: $show) {
                    _id title completed
                }
                remaining
            }
        }
        `;
      const variables = { show: show || "all" };
      const { data, errors } = await this.postQuery(query, variables, true);
      if (data) {
        this.todos = data.Todo.get;
        this.remaining = data.Todo.remaining;
      }
    },

    async fetchRemaining() {
      const query = `
        query {
            Todo {
                remaining
            }
        }
        `;
      const resp = await this.postQuery(query);
      this.remaining = resp.data.Todo.remaining;
    },

    async addTodo() {
      var value = this.newTodo && this.newTodo.trim();
      if (!value) {
        return;
      }
      const query = `
        query($todo: ITodo!) {
            Todo {
                add(todo: $todo) {
                    _id title completed
                }
            }
        }
        `;
      const variables = { todo: { title: value, completed: false } };
      const resp = await this.postQuery(query, variables);
      if (this.visibility !== "completed") this.todos.push(resp.data.Todo.add);
      this.remaining += 1;
      this.newTodo = "";
    },

    async login() {
      if (!this.credentials.username) return alert("Enter your username!");
      if (!this.credentials.password) return alert("Enter your password!");
      const query = `
        mutation($credentials: ICredentials!) {
            login(credentials: $credentials) {
                username
            }
        }
      `;
      const variables = { credentials: this.credentials };
      const { data } = await this.postQuery(query, variables, false);
      this.user = data.login;
    },

    async signup() {
      if (!this.credentials.username) return alert("Enter your username!");
      if (!this.credentials.password) return alert("Enter your password!");
      if (!this.credentials.cpassword) return alert("Confirm your password!");
      if (!this.credentials.password !== credentials.cpassword)
        return alert("Provided passwords do not match!");
      const query = `
        mutation($credentials: ICredentials!) {
            User {
                new(credentials: $credentials)
            }
        }
      `;
      const variables = { credentials: this.credentials };
      await this.postQuery(query, variables, false);
      this.visibility = "login";
    },

    removeTodo: function(todo) {
      this.todos.splice(this.todos.indexOf(todo), 1);
    },

    editTodo: function(todo) {
      this.beforeEditCache = todo.title;
      this.editedTodo = todo;
    },

    doneEdit: function(todo) {
      if (!this.editedTodo) {
        return;
      }
      this.editedTodo = null;
      todo.title = todo.title.trim();
      if (!todo.title) {
        this.removeTodo(todo);
      }
    },

    cancelEdit: function(todo) {
      this.editedTodo = null;
      todo.title = this.beforeEditCache;
    },

    removeCompleted: function() {
      this.todos = filters.active(this.todos);
    }
  },

  // a custom directive to wait for the DOM to be updated
  // before focusing on the input field.
  // http://vuejs.org/guide/custom-directive.html
  directives: {
    "todo-focus": function(el, binding) {
      if (binding.value) {
        el.focus();
      }
    }
  }
});

// handle routing
function onHashChange() {
  var visibility = window.location.hash.replace(/#\/?/, "");
  if (
    ["all", "active", "completed", "login", "signup"].indexOf(visibility) !== -1
  ) {
    app.visibility = visibility;
  } else {
    window.location.hash = "";
    app.visibility = "login";
  }
}

window.addEventListener("hashchange", onHashChange);
onHashChange();

// mount
app.$mount(".todoapp");
