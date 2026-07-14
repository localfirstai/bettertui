use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

use bettertui_engine::input::FocusManager;
use bettertui_engine::layout::LayoutProps;
use bettertui_engine::scheduler::Scheduler;
use bettertui_engine::tree::{NodeArena, NodeId, NodeKind, RenderNode, Style};
use bettertui_widgets::{
    Pipeline, Widget, WidgetContext, WidgetHost, WidgetId, WidgetRegistry, WidgetTree,
    reconcile::Reconciler, theme::Theme,
};

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn make_arena_and_ids(count: usize) -> (NodeArena, Vec<WidgetId>, Vec<NodeId>) {
    let mut arena = NodeArena::new();
    let mut wids = Vec::with_capacity(count);
    let mut nids = Vec::with_capacity(count);
    for _ in 0..count {
        let nid = arena.insert(RenderNode::new(NodeKind::Box));
        nids.push(nid);
        wids.push(WidgetId(nid));
    }
    (arena, wids, nids)
}

fn make_host_with_context() -> (WidgetHost, NodeArena, FocusManager, Scheduler, Theme) {
    (
        WidgetHost::new(),
        NodeArena::new(),
        FocusManager::new(),
        Scheduler::new(),
        Theme::default(),
    )
}

struct BenchWidget;

impl Widget for BenchWidget {
    fn kind(&self) -> &'static str {
        "Bench"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let id = ctx.make_box(LayoutProps::default(), Style::default());
        WidgetId(id)
    }
}

fn make_tree_with(
    tree: &mut WidgetTree,
    arena: &mut NodeArena,
    count: usize,
    branching: usize,
) -> WidgetId {
    let root_nid = arena.insert(RenderNode::new(NodeKind::Box));
    let root_wid = WidgetId(root_nid);
    tree.insert(root_wid, root_nid, "Box");

    let mut queue = vec![root_wid];
    let mut created = 0;
    while let Some(parent) = queue.pop() {
        for _ in 0..branching {
            if created >= count {
                break;
            }
            let nid = arena.insert(RenderNode::new(NodeKind::Text));
            let wid = WidgetId(nid);
            tree.insert(wid, nid, "Text");
            tree.set_parent(wid, parent);
            queue.push(wid);
            created += 1;
        }
        if created >= count {
            break;
        }
    }
    root_wid
}

/// Build two identical trees by recreating the same structure.
fn build_identical_trees(
    count: usize,
    branching: usize,
) -> (NodeArena, WidgetTree, WidgetTree, WidgetId) {
    let mut arena = NodeArena::new();
    let mut tree_a = WidgetTree::new();
    let root_a = make_tree_with(&mut tree_a, &mut arena, count, branching);

    // Build an identical tree in a separate arena
    let mut arena_b = NodeArena::new();
    let mut tree_b = WidgetTree::new();
    let _root_b = make_tree_with(&mut tree_b, &mut arena_b, count, branching);

    // Use arena_a's root as the common reconciler root
    (arena, tree_a, tree_b, root_a)
}

// ─── WidgetHost Benchmarks ───────────────────────────────────────────────────

fn bench_widget_host(c: &mut Criterion) {
    let mut group = c.benchmark_group("widgets/host");

    group.bench_function("mount_single", |b| {
        b.iter_with_setup(
            make_host_with_context,
            |(mut host, mut arena, mut focus, mut sched, theme)| {
                let mut ctx = WidgetContext {
                    arena: &mut arena,
                    focus_manager: &mut focus,
                    scheduler: &mut sched,
                    terminal_size: (80, 24),
                    theme: &theme,
                };
                let _wid = host.mount(Box::new(BenchWidget), &mut ctx);
                black_box(host.widget_count());
            },
        );
    });

    group.bench_function("mount_unmount_sequential", |b| {
        b.iter_with_setup(
            make_host_with_context,
            |(mut host, mut arena, mut focus, mut sched, theme)| {
                let mut ctx = WidgetContext {
                    arena: &mut arena,
                    focus_manager: &mut focus,
                    scheduler: &mut sched,
                    terminal_size: (80, 24),
                    theme: &theme,
                };
                for _ in 0..10 {
                    let wid = host.mount(Box::new(BenchWidget), &mut ctx);
                    host.unmount(wid, &mut ctx);
                }
                black_box(host.widget_count());
            },
        );
    });

    let sizes = [10, 100, 500];
    for size in &sizes {
        group.bench_with_input(BenchmarkId::new("mount_batch", size), size, |b, &size| {
            b.iter(|| {
                let (mut host, mut arena, mut focus, mut sched, theme) = make_host_with_context();
                let mut ctx = WidgetContext {
                    arena: &mut arena,
                    focus_manager: &mut focus,
                    scheduler: &mut sched,
                    terminal_size: (80, 24),
                    theme: &theme,
                };
                for _ in 0..size {
                    host.mount(Box::new(BenchWidget), &mut ctx);
                }
                black_box(host.widget_count());
            });
        });
    }

    group.finish();
}

// ─── WidgetTree Benchmarks ────────────────────────────────────────────────────

fn bench_widget_tree(c: &mut Criterion) {
    let mut group = c.benchmark_group("widgets/tree");

    group.bench_function("insert_single", |b| {
        b.iter_with_setup(
            || make_arena_and_ids(1),
            |(_arena, wids, nids)| {
                let mut tree = WidgetTree::new();
                tree.insert(wids[0], nids[0], "Box");
                black_box(tree.len());
            },
        );
    });

    for size in [10, 100, 1000] {
        group.bench_with_input(BenchmarkId::new("insert_batch", size), &size, |b, &size| {
            b.iter_with_setup(
                || make_arena_and_ids(size),
                |(_arena, wids, nids)| {
                    let mut tree = WidgetTree::new();
                    for i in 0..size {
                        tree.insert(wids[i], nids[i], "Box");
                    }
                    black_box(tree.len());
                },
            );
        });
    }

    group.bench_function("get_by_id", |b| {
        let (_arena, wids, nids) = make_arena_and_ids(100);
        let mut tree = WidgetTree::new();
        for i in 0..100 {
            tree.insert(wids[i], nids[i], "Box");
        }
        b.iter(|| {
            for wid in &wids {
                let _ = black_box(tree.get(*wid));
            }
        });
    });

    group.bench_function("traverse_all", |b| {
        let (_arena, wids, nids) = make_arena_and_ids(100);
        let mut tree = WidgetTree::new();
        for i in 0..100 {
            tree.insert(wids[i], nids[i], "Box");
        }
        b.iter(|| {
            let count = tree.iter().count();
            black_box(count);
        });
    });

    for size in [10, 100] {
        group.bench_with_input(
            BenchmarkId::new("build_parent_child_tree", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut arena = NodeArena::new();
                    let mut tree = WidgetTree::new();
                    let _root = make_tree_with(&mut tree, &mut arena, size, 4);
                    black_box(tree.len());
                });
            },
        );
    }

    for size in [10, 100] {
        group.bench_with_input(BenchmarkId::new("remove_all", size), &size, |b, &size| {
            let (_arena, wids, nids) = make_arena_and_ids(size);
            let mut tree = WidgetTree::new();
            for i in 0..size {
                tree.insert(wids[i], nids[i], "Box");
            }
            b.iter(|| {
                for wid in &wids {
                    tree.remove(*wid);
                }
                black_box(tree.is_empty());
            });
        });
    }

    group.finish();
}

// ─── WidgetRegistry Benchmarks ────────────────────────────────────────────────

fn bench_widget_registry(c: &mut Criterion) {
    let mut group = c.benchmark_group("widgets/registry");

    group.bench_function("register_single", |b| {
        b.iter(|| {
            let mut reg = WidgetRegistry::new();
            reg.register("Bench", || Box::new(BenchWidget));
            black_box(reg.len());
        });
    });

    for count in [10, 50, 200] {
        group.bench_with_input(
            BenchmarkId::new("register_batch", count),
            &count,
            |b, &count| {
                b.iter(|| {
                    let mut reg = WidgetRegistry::new();
                    for i in 0..count {
                        reg.register(Box::leak(format!("Bench_{}", i).into_boxed_str()), || {
                            Box::new(BenchWidget)
                        });
                    }
                    black_box(reg.len());
                });
            },
        );
    }

    group.bench_function("lookup_exists", |b| {
        let mut reg = WidgetRegistry::new();
        reg.register("Bench", || Box::new(BenchWidget));
        b.iter(|| {
            let result = reg.create("Bench");
            black_box(result.is_some());
        });
    });

    group.bench_function("lookup_missing", |b| {
        let reg = WidgetRegistry::new();
        b.iter(|| {
            let result = reg.create("NonExistent");
            black_box(result.is_none());
        });
    });

    group.bench_function("has_check", |b| {
        let mut reg = WidgetRegistry::new();
        reg.register("A", || Box::new(BenchWidget));
        reg.register("B", || Box::new(BenchWidget));
        reg.register("C", || Box::new(BenchWidget));
        b.iter(|| {
            let _ = black_box(reg.has("A"));
            let _ = black_box(reg.has("Z"));
        });
    });

    group.bench_function("kinds_collect", |b| {
        let mut reg = WidgetRegistry::new();
        for ch in 'A'..='Z' {
            let name = Box::leak(format!("Widget_{ch}").into_boxed_str());
            reg.register(name, || Box::new(BenchWidget));
        }
        b.iter(|| {
            let kinds = reg.kinds();
            black_box(kinds.len());
        });
    });

    group.finish();
}

// ─── Reconciler Benchmarks ────────────────────────────────────────────────────

fn bench_reconciler(c: &mut Criterion) {
    let mut group = c.benchmark_group("widgets/reconciler");

    fn make_deep_tree(
        arena: &mut NodeArena,
        tree: &mut WidgetTree,
        depth: usize,
        fanout: usize,
    ) -> WidgetId {
        let root_nid = arena.insert(RenderNode::new(NodeKind::Box));
        let root_wid = WidgetId(root_nid);
        tree.insert(root_wid, root_nid, "Box");

        let mut level = vec![root_wid];
        for _d in 0..depth {
            let mut next = Vec::new();
            for parent in &level {
                for _f in 0..fanout {
                    let nid = arena.insert(RenderNode::new(NodeKind::Box));
                    let wid = WidgetId(nid);
                    tree.insert(wid, nid, "Box");
                    tree.set_parent(wid, *parent);
                    next.push(wid);
                }
            }
            level = next;
        }
        root_wid
    }

    group.bench_function("identical_small", |b| {
        // Build two identical trees from scratch
        let (_arena_a, tree_a, tree_b, root_a) = build_identical_trees(10, 3);
        let mut reconciler = Reconciler::new();
        b.iter(|| {
            let ops = reconciler.reconcile(black_box(&tree_a), black_box(&tree_b), root_a);
            black_box(ops.len());
        });
    });

    for node_count in [10, 100] {
        group.bench_with_input(
            BenchmarkId::new("identical_medium", node_count),
            &node_count,
            |b, &count| {
                let (_arena, tree_a, tree_b, root) = build_identical_trees(count, 4);
                let mut reconciler = Reconciler::new();
                b.iter(|| {
                    let ops = reconciler.reconcile(black_box(&tree_a), black_box(&tree_b), root);
                    black_box(ops.len());
                });
            },
        );
    }

    group.bench_function("one_added", |b| {
        let mut arena = NodeArena::new();
        let mut old_tree = WidgetTree::new();
        let root = make_deep_tree(&mut arena, &mut old_tree, 2, 3);

        // Build new_tree by re-inserting all old_tree entries + one new
        let mut new_tree = WidgetTree::new();
        let root_nid = arena.insert(RenderNode::new(NodeKind::Box));
        let root_nwid = WidgetId(root_nid);
        new_tree.insert(root_nwid, root_nid, "Box");
        let child_nid = arena.insert(RenderNode::new(NodeKind::Text));
        let child_wid = WidgetId(child_nid);
        new_tree.insert(child_wid, child_nid, "Text");
        new_tree.set_parent(child_wid, root_nwid);

        let mut reconciler = Reconciler::new();
        b.iter(|| {
            let ops = reconciler.reconcile(black_box(&old_tree), black_box(&new_tree), root);
            black_box(ops.len());
        });
    });

    group.bench_function("one_removed", |b| {
        let mut arena = NodeArena::new();
        let mut old_tree = WidgetTree::new();
        let root = make_deep_tree(&mut arena, &mut old_tree, 2, 3);

        // Build new_tree without one child
        let mut new_tree = WidgetTree::new();
        let root_nid = arena.insert(RenderNode::new(NodeKind::Box));
        let root_nwid = WidgetId(root_nid);
        new_tree.insert(root_nwid, root_nid, "Box");

        let mut reconciler = Reconciler::new();
        b.iter(|| {
            let ops = reconciler.reconcile(black_box(&old_tree), black_box(&new_tree), root);
            black_box(ops.len());
        });
    });

    group.finish();
}

// ─── Pipeline Benchmarks ─────────────────────────────────────────────────────

fn bench_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("widgets/pipeline");

    group.bench_function("new", |b| {
        b.iter(|| {
            let p = Pipeline::new();
            black_box(p.needs_render());
        });
    });

    group.bench_function("mark_clear_dirty", |b| {
        let mut p = Pipeline::new();
        b.iter(|| {
            p.mark_dirty();
            black_box(p.needs_render());
            p.clear_dirty();
            black_box(!p.needs_render());
        });
    });

    group.bench_function("advance_generation", |b| {
        let mut p = Pipeline::new();
        b.iter(|| {
            let generation = p.advance_generation();
            black_box(generation);
        });
    });

    group.finish();
}

// ─── Theme Benchmarks ────────────────────────────────────────────────────────

fn bench_theme(c: &mut Criterion) {
    let mut group = c.benchmark_group("widgets/theme");

    group.bench_function("create_default", |b| {
        b.iter(|| {
            let theme = Theme::default();
            black_box(theme.name.as_ref());
        });
    });

    group.bench_function("colors_access_all_fields", |b| {
        let theme = Theme::default();
        b.iter(|| {
            let _ = black_box(theme.colors.background);
            let _ = black_box(theme.colors.primary);
            let _ = black_box(theme.colors.text);
            let _ = black_box(theme.colors.border);
            let _ = black_box(theme.colors.accent);
        });
    });

    group.bench_function("spacing_access", |b| {
        let theme = Theme::default();
        b.iter(|| {
            let _ = black_box(theme.spacing.sm);
            let _ = black_box(theme.spacing.md);
            let _ = black_box(theme.spacing.lg);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_widget_host,
    bench_widget_tree,
    bench_widget_registry,
    bench_reconciler,
    bench_pipeline,
    bench_theme,
);
criterion_main!(benches);
