(function() {var implementors = {};
implementors["eventstore_tcp"] = ["impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="eventstore_tcp/raw/client_messages/mod_ReadEventCompleted/enum.ReadEventResult.html" title="enum eventstore_tcp::raw::client_messages::mod_ReadEventCompleted::ReadEventResult">ReadEventResult</a>","impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="eventstore_tcp/raw/client_messages/mod_ReadStreamEventsCompleted/enum.ReadStreamResult.html" title="enum eventstore_tcp::raw::client_messages::mod_ReadStreamEventsCompleted::ReadStreamResult">ReadStreamResult</a>","impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="eventstore_tcp/raw/client_messages/mod_ReadAllEventsCompleted/enum.ReadAllResult.html" title="enum eventstore_tcp::raw::client_messages::mod_ReadAllEventsCompleted::ReadAllResult">ReadAllResult</a>","impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="eventstore_tcp/raw/client_messages/mod_UpdatePersistentSubscriptionCompleted/enum.UpdatePersistentSubscriptionResult.html" title="enum eventstore_tcp::raw::client_messages::mod_UpdatePersistentSubscriptionCompleted::UpdatePersistentSubscriptionResult">UpdatePersistentSubscriptionResult</a>","impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="eventstore_tcp/raw/client_messages/mod_CreatePersistentSubscriptionCompleted/enum.CreatePersistentSubscriptionResult.html" title="enum eventstore_tcp::raw::client_messages::mod_CreatePersistentSubscriptionCompleted::CreatePersistentSubscriptionResult">CreatePersistentSubscriptionResult</a>","impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="eventstore_tcp/raw/client_messages/mod_DeletePersistentSubscriptionCompleted/enum.DeletePersistentSubscriptionResult.html" title="enum eventstore_tcp::raw::client_messages::mod_DeletePersistentSubscriptionCompleted::DeletePersistentSubscriptionResult">DeletePersistentSubscriptionResult</a>","impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="eventstore_tcp/raw/client_messages/mod_PersistentSubscriptionNakEvents/enum.NakAction.html" title="enum eventstore_tcp::raw::client_messages::mod_PersistentSubscriptionNakEvents::NakAction">NakAction</a>","impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="eventstore_tcp/raw/client_messages/mod_SubscriptionDropped/enum.SubscriptionDropReason.html" title="enum eventstore_tcp::raw::client_messages::mod_SubscriptionDropped::SubscriptionDropReason">SubscriptionDropReason</a>","impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="eventstore_tcp/raw/client_messages/mod_NotHandled/enum.NotHandledReason.html" title="enum eventstore_tcp::raw::client_messages::mod_NotHandled::NotHandledReason">NotHandledReason</a>","impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="eventstore_tcp/raw/client_messages/mod_ScavengeDatabaseCompleted/enum.ScavengeResult.html" title="enum eventstore_tcp::raw::client_messages::mod_ScavengeDatabaseCompleted::ScavengeResult">ScavengeResult</a>","impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="eventstore_tcp/raw/client_messages/enum.OperationResult.html" title="enum eventstore_tcp::raw::client_messages::OperationResult">OperationResult</a>","impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="eventstore_tcp/adapted/enum.WriteEventsFailure.html" title="enum eventstore_tcp::adapted::WriteEventsFailure">WriteEventsFailure</a>","impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="struct" href="eventstore_tcp/codec/struct.TcpFlags.html" title="struct eventstore_tcp::codec::TcpFlags">TcpFlags</a>","impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="eventstore_tcp/enum.ReadDirection.html" title="enum eventstore_tcp::ReadDirection">ReadDirection</a>","impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="eventstore_tcp/enum.ExpectedVersion.html" title="enum eventstore_tcp::ExpectedVersion">ExpectedVersion</a>","impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="struct" href="eventstore_tcp/struct.StreamVersion.html" title="struct eventstore_tcp::StreamVersion">StreamVersion</a>","impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="eventstore_tcp/enum.EventNumber.html" title="enum eventstore_tcp::EventNumber">EventNumber</a>","impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="eventstore_tcp/enum.ContentType.html" title="enum eventstore_tcp::ContentType">ContentType</a>","impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="eventstore_tcp/enum.LogPosition.html" title="enum eventstore_tcp::LogPosition">LogPosition</a>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
