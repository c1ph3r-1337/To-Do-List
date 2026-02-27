mod task;
mod storage;

use gtk::prelude::*;
use gtk::*;
use std::cell::RefCell;
use std::rc::Rc;

use task::Task;
use storage::{load_tasks, save_tasks};

fn main() {
    let app = Application::builder()
        .application_id("com.todo.gtk")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    load_css();

    let tasks = Rc::new(RefCell::new(load_tasks()));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Rust GTK TODO")
        .default_width(420)
        .default_height(720)
        .resizable(false)
        .decorated(false) 
        .build();

    let main_box = Box::new(Orientation::Vertical, 15);
    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);

    let entry = Entry::new();
    entry.set_placeholder_text(Some("Enter task title..."));

    let calendar = Calendar::new();

    let add_button = Button::with_label("Add Task");
    add_button.add_css_class("suggested-action");

    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);

    let list_box = ListBox::new();
    list_box.set_selection_mode(SelectionMode::None);

    scrolled.set_child(Some(&list_box));

    let expander = Expander::builder()
    .label("Select Date")
    .expanded(false)   // collapsed by default
    .build();

    expander.set_child(Some(&calendar));

    main_box.append(&expander);
    main_box.append(&entry);
    main_box.append(&add_button);
    main_box.append(&scrolled);

    window.set_child(Some(&main_box));

    refresh_list(&list_box, &tasks);

    // ADD BUTTON
    {
        let tasks = Rc::clone(&tasks);
        let list_box = list_box.clone();
        add_button.connect_clicked(move |_| {
            let title = entry.text().to_string();
            if title.is_empty() {
                return;
            }

            let datetime = calendar.date();
            let due_date = format!(
                "{:04}-{:02}-{:02}",
                datetime.year(),
                datetime.month(),
                datetime.day_of_month()
            );

            {
                let mut task_list = tasks.borrow_mut();
                let id = task_list.len() as u32 + 1;
                task_list.push(Task::new(id, title, due_date));
                save_tasks(&task_list).unwrap();
            } // borrow ends here

            entry.set_text("");
            refresh_list(&list_box, &tasks);
        });
    }

    window.show();
    window.set_size_request(420, 720);
}

fn refresh_list(list_box: &ListBox, tasks: &Rc<RefCell<Vec<Task>>>) {
    while let Some(child) = list_box.first_child() {
        list_box.remove(&child);
    }

    let snapshot = tasks.borrow().clone(); // safe borrow

    for task in snapshot {
        let row = ListBoxRow::new();

        let card = Box::new(Orientation::Horizontal, 10);
        card.add_css_class("card");
        card.set_margin_top(5);
        card.set_margin_bottom(5);
        card.set_margin_start(5);
        card.set_margin_end(5);

        let checkbox = CheckButton::new();
        checkbox.set_active(task.completed);

        let title = format!("{} ({})", task.title, task.date);

        let label = Label::new(Some(&title));
        label.set_hexpand(true);
        label.set_xalign(0.0);

        if task.completed {
            label.add_css_class("dim-label");
        }

        let delete_btn = Button::with_label("🗑");

        let task_id = task.id;

        // COMPLETE TOGGLE
        {
            let tasks = Rc::clone(tasks);
            let list_box = list_box.clone();
            checkbox.connect_toggled(move |chk| {
                {
                    let mut task_list = tasks.borrow_mut();
                    if let Some(t) = task_list.iter_mut().find(|t| t.id == task_id) {
                        t.completed = chk.is_active();
                    }
                    save_tasks(&task_list).unwrap();
                }
                refresh_list(&list_box, &tasks);
            });
        }

        // DELETE
        {
            let tasks = Rc::clone(tasks);
            let list_box = list_box.clone();
            delete_btn.connect_clicked(move |_| {
                {
                    let mut task_list = tasks.borrow_mut();
                    task_list.retain(|t| t.id != task_id);

                    for (i, t) in task_list.iter_mut().enumerate() {
                        t.id = (i + 1) as u32;
                    }

                    save_tasks(&task_list).unwrap();
                }
                refresh_list(&list_box, &tasks);
            });
        }

        card.append(&checkbox);
        card.append(&label);
        card.append(&delete_btn);

        row.set_child(Some(&card));
        list_box.append(&row);
    }
}

fn load_css() {
    let provider = CssProvider::new();

    provider.load_from_data(
        "
        window {
            background: rgba(30, 20, 60, 0.35);
        }

        .task-card {
            background: rgba(255,255,255,0.08);
            border-radius: 16px;
            padding: 14px;
            color: white;
            backdrop-filter: blur(10px);
        }

        entry {
            background: rgba(255,255,255,0.10);
            border-radius: 12px;
            padding: 12px;
            color: white;
            border: none;
        }

        button {
            background: rgba(180,150,255,0.25);
            border-radius: 14px;
            padding: 12px;
            color: white;
            font-weight: 600;
        }
        "
    );

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}