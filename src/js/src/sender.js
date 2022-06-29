import { LocalNotifications } from "@capacitor/local-notifications";

export async function start_local_notification() {
    LocalNotifications.registerActionTypes({
        types: [
            {
                id: "test_actions",
                actions: [
                    {
                        id: "view",
                        title: "Open App"
                    },
                    {
                        id: "remove",
                        title: "Remove",
                        destructive: true
                    },
                    {
                        id: "respond",
                        title: "Respond",
                        input: true
                    }
                ]
            }
        ]
    })

    const status = await LocalNotifications.schedule(
        {
            notifications: [
                {
                    title: "Local as fk",
                    id: 1,
                    body: "Local Notifications ;)",
                    actionTypeId: "test_actions"
                }
            ]
        }
    );
    // console.log(status.notifications);
}

