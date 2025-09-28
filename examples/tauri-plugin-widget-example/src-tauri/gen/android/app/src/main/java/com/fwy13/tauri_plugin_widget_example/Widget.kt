package com.fwy13.tauri_plugin_widget_example

import android.app.PendingIntent
import android.appwidget.AppWidgetManager
import android.appwidget.AppWidgetProvider
import android.content.Context
import android.content.Intent
import android.widget.RemoteViews

class Widget : AppWidgetProvider() {
  override fun onUpdate(
    context: Context,
    appWidgetManager: AppWidgetManager,
    appWidgetIds: IntArray
  ) {
    val prefs = context.getSharedPreferences("com.fwy13.tauri_plugin_widget_example.Widget", Context.MODE_PRIVATE)

    for (appWidgetId in appWidgetIds) {
      val text = prefs.getString("widget_text", "") ?: ""

      val views = RemoteViews(context.packageName, R.layout.widget_bride)
      views.setTextViewText(R.id.widget_text, text)

      val intent = Intent(context, MainActivity::class.java)
      val pendingIntent = PendingIntent.getActivity(
        context,
        0,
        intent,
        PendingIntent.FLAG_IMMUTABLE or PendingIntent.FLAG_UPDATE_CURRENT
      )
      views.setOnClickPendingIntent(R.id.widget_text, pendingIntent)

      appWidgetManager.updateAppWidget(appWidgetId, views)
    }
  }
}