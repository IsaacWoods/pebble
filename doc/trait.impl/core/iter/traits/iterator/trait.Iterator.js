(function() {var implementors = {
"bitflags":[["impl&lt;B: <a class=\"trait\" href=\"bitflags/trait.Flags.html\" title=\"trait bitflags::Flags\">Flags</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"bitflags/iter/struct.IterNames.html\" title=\"struct bitflags::iter::IterNames\">IterNames</a>&lt;B&gt;"],["impl&lt;B: <a class=\"trait\" href=\"bitflags/trait.Flags.html\" title=\"trait bitflags::Flags\">Flags</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"bitflags/iter/struct.Iter.html\" title=\"struct bitflags::iter::Iter\">Iter</a>&lt;B&gt;"]],
"cordyceps":[["impl&lt;'list, T: <a class=\"trait\" href=\"cordyceps/trait.Linked.html\" title=\"trait cordyceps::Linked\">Linked</a>&lt;<a class=\"struct\" href=\"cordyceps/list/struct.Links.html\" title=\"struct cordyceps::list::Links\">Links</a>&lt;T&gt;&gt; + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"cordyceps/list/struct.Cursor.html\" title=\"struct cordyceps::list::Cursor\">Cursor</a>&lt;'list, T&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"cordyceps/struct.Stack.html\" title=\"struct cordyceps::Stack\">Stack</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"cordyceps/trait.Linked.html\" title=\"trait cordyceps::Linked\">Linked</a>&lt;<a class=\"struct\" href=\"cordyceps/stack/struct.Links.html\" title=\"struct cordyceps::stack::Links\">Links</a>&lt;T&gt;&gt;,</div>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"cordyceps/mpsc_queue/struct.Consumer.html\" title=\"struct cordyceps::mpsc_queue::Consumer\">Consumer</a>&lt;'_, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"cordyceps/trait.Linked.html\" title=\"trait cordyceps::Linked\">Linked</a>&lt;<a class=\"struct\" href=\"cordyceps/mpsc_queue/struct.Links.html\" title=\"struct cordyceps::mpsc_queue::Links\">Links</a>&lt;T&gt;&gt;,</div>"],["impl&lt;T: <a class=\"trait\" href=\"cordyceps/trait.Linked.html\" title=\"trait cordyceps::Linked\">Linked</a>&lt;<a class=\"struct\" href=\"cordyceps/list/struct.Links.html\" title=\"struct cordyceps::list::Links\">Links</a>&lt;T&gt;&gt; + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"cordyceps/list/struct.IntoIter.html\" title=\"struct cordyceps::list::IntoIter\">IntoIter</a>&lt;T&gt;"],["impl&lt;'list, T: <a class=\"trait\" href=\"cordyceps/trait.Linked.html\" title=\"trait cordyceps::Linked\">Linked</a>&lt;<a class=\"struct\" href=\"cordyceps/list/struct.Links.html\" title=\"struct cordyceps::list::Links\">Links</a>&lt;T&gt;&gt; + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"cordyceps/list/struct.Iter.html\" title=\"struct cordyceps::list::Iter\">Iter</a>&lt;'list, T&gt;"],["impl&lt;'list, T: <a class=\"trait\" href=\"cordyceps/trait.Linked.html\" title=\"trait cordyceps::Linked\">Linked</a>&lt;<a class=\"struct\" href=\"cordyceps/list/struct.Links.html\" title=\"struct cordyceps::list::Links\">Links</a>&lt;T&gt;&gt; + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"cordyceps/list/struct.IterMut.html\" title=\"struct cordyceps::list::IterMut\">IterMut</a>&lt;'list, T&gt;"],["impl&lt;T, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"cordyceps/list/struct.DrainFilter.html\" title=\"struct cordyceps::list::DrainFilter\">DrainFilter</a>&lt;'_, T, F&gt;<div class=\"where\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;T</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a>,\n    T: <a class=\"trait\" href=\"cordyceps/trait.Linked.html\" title=\"trait cordyceps::Linked\">Linked</a>&lt;<a class=\"struct\" href=\"cordyceps/list/struct.Links.html\" title=\"struct cordyceps::list::Links\">Links</a>&lt;T&gt;&gt; + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div>"],["impl&lt;'list, T: <a class=\"trait\" href=\"cordyceps/trait.Linked.html\" title=\"trait cordyceps::Linked\">Linked</a>&lt;<a class=\"struct\" href=\"cordyceps/list/struct.Links.html\" title=\"struct cordyceps::list::Links\">Links</a>&lt;T&gt;&gt; + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"cordyceps/list/struct.CursorMut.html\" title=\"struct cordyceps::list::CursorMut\">CursorMut</a>&lt;'list, T&gt;"]],
"heapless":[["impl&lt;'a, K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"heapless/struct.IndexMapValues.html\" title=\"struct heapless::IndexMapValues\">Values</a>&lt;'a, K, V&gt;"],["impl&lt;'a, T, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"heapless/spsc/struct.IterMut.html\" title=\"struct heapless::spsc::IterMut\">IterMut</a>&lt;'a, T, N&gt;"],["impl&lt;'a, K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"heapless/struct.IndexMapKeys.html\" title=\"struct heapless::IndexMapKeys\">Keys</a>&lt;'a, K, V&gt;"],["impl&lt;'a, T, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"heapless/spsc/struct.Iter.html\" title=\"struct heapless::spsc::Iter\">Iter</a>&lt;'a, T, N&gt;"],["impl&lt;'a, K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"heapless/struct.IndexMapValuesMut.html\" title=\"struct heapless::IndexMapValuesMut\">ValuesMut</a>&lt;'a, K, V&gt;"],["impl&lt;'a, T, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"heapless/struct.OldestOrdered.html\" title=\"struct heapless::OldestOrdered\">OldestOrdered</a>&lt;'a, T, N&gt;"],["impl&lt;'a, T, Idx, K, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"heapless/sorted_linked_list/struct.Iter.html\" title=\"struct heapless::sorted_linked_list::Iter\">Iter</a>&lt;'a, T, Idx, K, N&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a>,\n    Idx: <a class=\"trait\" href=\"heapless/sorted_linked_list/trait.SortedLinkedListIndex.html\" title=\"trait heapless::sorted_linked_list::SortedLinkedListIndex\">SortedLinkedListIndex</a>,\n    K: <a class=\"trait\" href=\"heapless/sorted_linked_list/trait.Kind.html\" title=\"trait heapless::sorted_linked_list::Kind\">Kind</a>,</div>"],["impl&lt;'a, K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"heapless/struct.IndexMapIter.html\" title=\"struct heapless::IndexMapIter\">Iter</a>&lt;'a, K, V&gt;"],["impl&lt;'a, K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"heapless/struct.IndexMapIterMut.html\" title=\"struct heapless::IndexMapIterMut\">IterMut</a>&lt;'a, K, V&gt;"],["impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"heapless/struct.IndexSetIter.html\" title=\"struct heapless::IndexSetIter\">Iter</a>&lt;'a, T&gt;"]],
"mycelium_util":[["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"mycelium_util/error/struct.ErrorIter.html\" title=\"struct mycelium_util::error::ErrorIter\">ErrorIter</a>&lt;'a&gt;"],["impl&lt;R: <a class=\"trait\" href=\"mycelium_util/io/trait.Read.html\" title=\"trait mycelium_util::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"mycelium_util/io/struct.Bytes.html\" title=\"struct mycelium_util::io::Bytes\">Bytes</a>&lt;R&gt;"]],
"pci_types":[["impl&lt;'a, T: <a class=\"trait\" href=\"pci_types/trait.ConfigRegionAccess.html\" title=\"trait pci_types::ConfigRegionAccess\">ConfigRegionAccess</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"pci_types/capability/struct.CapabilityIterator.html\" title=\"struct pci_types::capability::CapabilityIterator\">CapabilityIterator</a>&lt;'a, T&gt;"]],
"proc_macro2":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"proc_macro2/token_stream/struct.IntoIter.html\" title=\"struct proc_macro2::token_stream::IntoIter\">IntoIter</a>"]],
"tracing_core":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"tracing_core/field/struct.Iter.html\" title=\"struct tracing_core::field::Iter\">Iter</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()