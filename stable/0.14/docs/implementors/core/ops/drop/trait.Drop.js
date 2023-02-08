(function() {var implementors = {};
implementors["cairo"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"cairo/struct.RectangleList.html\" title=\"struct cairo::RectangleList\">RectangleList</a>","synthetic":false,"types":["cairo::context::RectangleList"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"cairo/struct.Context.html\" title=\"struct cairo::Context\">Context</a>","synthetic":false,"types":["cairo::context::Context"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"cairo/struct.Device.html\" title=\"struct cairo::Device\">Device</a>","synthetic":false,"types":["cairo::device::Device"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"cairo/struct.ImageSurfaceData.html\" title=\"struct cairo::ImageSurfaceData\">ImageSurfaceData</a>&lt;'a&gt;","synthetic":false,"types":["cairo::image_surface::ImageSurfaceData"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"cairo/struct.Path.html\" title=\"struct cairo::Path\">Path</a>","synthetic":false,"types":["cairo::paths::Path"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"cairo/struct.Pattern.html\" title=\"struct cairo::Pattern\">Pattern</a>","synthetic":false,"types":["cairo::patterns::Pattern"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"cairo/struct.Region.html\" title=\"struct cairo::Region\">Region</a>","synthetic":false,"types":["cairo::region::Region"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"cairo/struct.Surface.html\" title=\"struct cairo::Surface\">Surface</a>","synthetic":false,"types":["cairo::surface::Surface"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"cairo/struct.MappedImageSurface.html\" title=\"struct cairo::MappedImageSurface\">MappedImageSurface</a>","synthetic":false,"types":["cairo::surface::MappedImageSurface"]}];
implementors["gio"] = [{"text":"impl&lt;F, O, T, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"gio/struct.GioFuture.html\" title=\"struct gio::GioFuture\">GioFuture</a>&lt;F, O, T, E&gt;","synthetic":false,"types":["gio::gio_future::GioFuture"]}];
implementors["glib"] = [{"text":"impl&lt;T:&nbsp;'static, MM:&nbsp;<a class=\"trait\" href=\"glib/boxed/trait.BoxedMemoryManager.html\" title=\"trait glib::boxed::BoxedMemoryManager\">BoxedMemoryManager</a>&lt;T&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/boxed/struct.Boxed.html\" title=\"struct glib::boxed::Boxed\">Boxed</a>&lt;T, MM&gt;","synthetic":false,"types":["glib::boxed::Boxed"]},{"text":"impl&lt;T, MM:&nbsp;<a class=\"trait\" href=\"glib/shared/trait.SharedMemoryManager.html\" title=\"trait glib::shared::SharedMemoryManager\">SharedMemoryManager</a>&lt;T&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/shared/struct.Shared.html\" title=\"struct glib::shared::Shared\">Shared</a>&lt;T, MM&gt;","synthetic":false,"types":["glib::shared::Shared"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/object/struct.ObjectRef.html\" title=\"struct glib::object::ObjectRef\">ObjectRef</a>","synthetic":false,"types":["glib::object::ObjectRef"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/object/struct.PropertyNotificationFreezeGuard.html\" title=\"struct glib::object::PropertyNotificationFreezeGuard\">PropertyNotificationFreezeGuard</a>","synthetic":false,"types":["glib::object::PropertyNotificationFreezeGuard"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"glib/object/trait.ObjectType.html\" title=\"trait glib::object::ObjectType\">ObjectType</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/object/struct.WeakRef.html\" title=\"struct glib::object::WeakRef\">WeakRef</a>&lt;T&gt;","synthetic":false,"types":["glib::object::WeakRef"]},{"text":"impl&lt;'a, T:&nbsp;<a class=\"trait\" href=\"glib/object/trait.IsClass.html\" title=\"trait glib::object::IsClass\">IsClass</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/object/struct.ClassRef.html\" title=\"struct glib::object::ClassRef\">ClassRef</a>&lt;'a, T&gt;","synthetic":false,"types":["glib::object::ClassRef"]},{"text":"impl&lt;'a, T:&nbsp;<a class=\"trait\" href=\"glib/object/trait.IsInterface.html\" title=\"trait glib::object::IsInterface\">IsInterface</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/object/struct.InterfaceRef.html\" title=\"struct glib::object::InterfaceRef\">InterfaceRef</a>&lt;'a, T&gt;","synthetic":false,"types":["glib::object::InterfaceRef"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/struct.EnumClass.html\" title=\"struct glib::EnumClass\">EnumClass</a>","synthetic":false,"types":["glib::enums::EnumClass"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/struct.FlagsClass.html\" title=\"struct glib::FlagsClass\">FlagsClass</a>","synthetic":false,"types":["glib::enums::FlagsClass"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/translate/struct.List.html\" title=\"struct glib::translate::List\">List</a>","synthetic":false,"types":["glib::translate::List"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/translate/struct.SList.html\" title=\"struct glib::translate::SList\">SList</a>","synthetic":false,"types":["glib::translate::SList"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/translate/struct.HashTable.html\" title=\"struct glib::translate::HashTable\">HashTable</a>","synthetic":false,"types":["glib::translate::HashTable"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/translate/struct.PtrArray.html\" title=\"struct glib::translate::PtrArray\">PtrArray</a>","synthetic":false,"types":["glib::translate::PtrArray"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/struct.GString.html\" title=\"struct glib::GString\">GString</a>","synthetic":false,"types":["glib::gstring::GString"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/struct.Sender.html\" title=\"struct glib::Sender\">Sender</a>&lt;T&gt;","synthetic":false,"types":["glib::main_context_channel::Sender"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/struct.SyncSender.html\" title=\"struct glib::SyncSender\">SyncSender</a>&lt;T&gt;","synthetic":false,"types":["glib::main_context_channel::SyncSender"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/struct.Receiver.html\" title=\"struct glib::Receiver\">Receiver</a>&lt;T&gt;","synthetic":false,"types":["glib::main_context_channel::Receiver"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/value/struct.Value.html\" title=\"struct glib::value::Value\">Value</a>","synthetic":false,"types":["glib::value::Value"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/value/struct.ValueArray.html\" title=\"struct glib::value::ValueArray\">ValueArray</a>","synthetic":false,"types":["glib::value::ValueArray"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/struct.VariantType.html\" title=\"struct glib::VariantType\">VariantType</a>","synthetic":false,"types":["glib::variant_type::VariantType"]},{"text":"impl&lt;'a, T:&nbsp;<a class=\"trait\" href=\"glib/send_unique/trait.SendUnique.html\" title=\"trait glib::send_unique::SendUnique\">SendUnique</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/send_unique/struct.Ref.html\" title=\"struct glib::send_unique::Ref\">Ref</a>&lt;'a, T&gt;","synthetic":false,"types":["glib::send_unique::Ref"]},{"text":"impl&lt;T, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/struct.SourceFuture.html\" title=\"struct glib::SourceFuture\">SourceFuture</a>&lt;T, F&gt;","synthetic":false,"types":["glib::source_futures::SourceFuture"]},{"text":"impl&lt;T, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/struct.SourceStream.html\" title=\"struct glib::SourceStream\">SourceStream</a>&lt;T, F&gt;","synthetic":false,"types":["glib::source_futures::SourceStream"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"glib/struct.ThreadPool.html\" title=\"struct glib::ThreadPool\">ThreadPool</a>","synthetic":false,"types":["glib::thread_pool::ThreadPool"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()