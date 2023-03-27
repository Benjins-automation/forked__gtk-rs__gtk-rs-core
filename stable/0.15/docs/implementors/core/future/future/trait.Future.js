(function() {var implementors = {
"gio":[["impl&lt;F, O, T, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"gio/struct.GioFuture.html\" title=\"struct gio::GioFuture\">GioFuture</a>&lt;F, O, T, E&gt;<span class=\"where fmt-newline\">where\n    O: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + 'static,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;O</a>, &amp;<a class=\"struct\" href=\"gio/struct.Cancellable.html\" title=\"struct gio::Cancellable\">Cancellable</a>, <a class=\"struct\" href=\"gio/struct.GioFutureResult.html\" title=\"struct gio::GioFutureResult\">GioFutureResult</a>&lt;T, E&gt;) + 'static,</span>"]],
"glib":[["impl&lt;F, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"glib/struct.SourceFuture.html\" title=\"struct glib::SourceFuture\">SourceFuture</a>&lt;F, T&gt;<span class=\"where fmt-newline\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(Sender&lt;T&gt;) -&gt; <a class=\"struct\" href=\"glib/struct.Source.html\" title=\"struct glib::Source\">Source</a> + 'static,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()