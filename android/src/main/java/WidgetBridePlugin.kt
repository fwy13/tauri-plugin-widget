package git.fwy13.widget

import android.app.Activity
import android.app.PendingIntent
import android.appwidget.AppWidgetManager
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.content.SharedPreferences
import android.os.Build
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin


@InvokeArg
class SetItemsRequest {
    var key: String = ""
    var value: String = ""
    var group: String = ""
}

@InvokeArg
class GetItemsRequest {
    var key: String = ""
    var group: String = ""
}
@InvokeArg
class SetRegisterWidgetRequest {
    var widgets: Array<String> = arrayOf("")
}
@InvokeArg
class ReloadTimelines {
    var of_kind : String = ""
}

@TauriPlugin
class WidgetBridePlugin(private val activity: Activity): Plugin(activity) {

    public final var listWidgetProvider: Array<String> = arrayOf("")

    private final fun getPrefs(group: String): SharedPreferences {
        return activity.getSharedPreferences(group, Context.MODE_PRIVATE);
    }

    @Command
    fun setItems(invoke: Invoke) {
        val args = invoke.parseArgs(SetItemsRequest::class.java)
        val editor = getPrefs(args.group).edit()
        editor.putString(args.key.toString(), args.value.toString())
        editor.apply()
        invoke.resolve(JSObject().put("results", true))
    }

    @Command
    fun getItems(invoke: Invoke) {
        val args = invoke.parseArgs(GetItemsRequest::class.java)
        val value = getPrefs(args.group).getString(args.key, null)
        invoke.resolve(JSObject().put("results", value))
    }

    @Command
    fun setRegisterWidget(invoke: Invoke) {
        val args = invoke.parseArgs(SetRegisterWidgetRequest::class.java)
        listWidgetProvider = args.widgets;
        invoke.resolve(JSObject().put("results", true))
    }

    @Command
    fun reloadAllTimelines(invoke: Invoke) {
        for (className: String in listWidgetProvider) {
            try {
                val widgetClass = Class.forName(className)
                val componentName = ComponentName(activity, className)
                val ids = AppWidgetManager.getInstance(activity).getAppWidgetIds(componentName)
                if (ids.size > 0) {
                    val updateIntent = Intent(activity, widgetClass)
                    updateIntent.action = AppWidgetManager.ACTION_APPWIDGET_UPDATE
                    updateIntent.putExtra(AppWidgetManager.EXTRA_APPWIDGET_IDS, ids)
                    activity.sendBroadcast(updateIntent)
                }
            } catch (
                e: ClassNotFoundException
            ) {
                e.printStackTrace();
            }
        }
        invoke.resolve(JSObject().put("results", true))
    }
    @Command
    fun reloadTimelines(invoke: Invoke) {
            val args = invoke.parseArgs(ReloadTimelines::class.java)
            val className = args.of_kind
            try {
                val widgetClass = Class.forName(className)
                val componentName = ComponentName(activity, className)
                val ids = AppWidgetManager.getInstance(activity).getAppWidgetIds(componentName)
                if (ids.size > 0) {
                    val updateIntent = Intent(activity, widgetClass)
                    updateIntent.action = AppWidgetManager.ACTION_APPWIDGET_UPDATE
                    updateIntent.putExtra(AppWidgetManager.EXTRA_APPWIDGET_IDS, ids)
                    activity.sendBroadcast(updateIntent)
                }
            } catch (
                e: ClassNotFoundException
            ) {
                e.printStackTrace();
            }
        invoke.resolve(JSObject().put("results", true))
    }
    @Command
    fun requestWidget(invoke: Invoke) {

        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.O) {
            invoke.reject("This feature requires Android O (API level 26) or higher.")
            return
        }

        if (listWidgetProvider.size == 0) {
            invoke.reject("No widget registered")
            return
        }

        val flags = PendingIntent.FLAG_UPDATE_CURRENT or
            (if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S)
                PendingIntent.FLAG_MUTABLE
            else
                PendingIntent.FLAG_IMMUTABLE)


        try {
            val widgetClass = Class.forName(listWidgetProvider[0])
            val componentName = ComponentName(activity, listWidgetProvider[0])
            val appWidgetManager = AppWidgetManager.getInstance(activity)
            if (appWidgetManager.isRequestPinAppWidgetSupported()) {
                val pinedWidgetCallbackIntent = Intent(activity, widgetClass)
                val successCallback = PendingIntent.getBroadcast(
                    activity,
0,
                    pinedWidgetCallbackIntent,
                    flags
                )
                appWidgetManager.requestPinAppWidget(componentName, null, successCallback)
                invoke.resolve(JSObject().put("results", true))

            } else {
                invoke.reject("Pinning not supported")
            }

        }catch (e: ClassNotFoundException) {
            invoke.reject("Error: Widget provider class not found: " + e.message);
            e.printStackTrace();
        } catch (e: IllegalArgumentException) {
            invoke.reject("Error: Invalid argument: " + e.message);
            e.printStackTrace();
        } catch (e: NullPointerException) {
            invoke.reject("Error: Null pointer encountered: " + e.message);
            e.printStackTrace();
        } catch (e: Exception) {
            invoke.reject("Unexpected error: " + e.message);
            e.printStackTrace();
        }

    }
}