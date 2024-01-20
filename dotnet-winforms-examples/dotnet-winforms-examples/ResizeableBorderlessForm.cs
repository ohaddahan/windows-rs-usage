using System;
using System.Windows.Forms;

namespace dotnet_winforms_examples
{
	public partial class ResizeableBorderlessForm : Form
	{
		public ResizeableBorderlessForm()
		{
			InitializeComponent();
		}

		#region Resize (Horizontal + Vertical)

		bool Resizing;
		bool VerticalResizing;
		bool HorizontalResizing;
		(int X, int Y) ResizingMouseOrigin;

		private void Resize_MouseDown(object sender, MouseEventArgs e)
		{
			if (WindowState is FormWindowState.Normal)
			{
				Resizing = true;
				((Control)sender).Capture = true;
				ResizingMouseOrigin = (e.X, e.Y);
				VerticalResizing = sender == resizePanel || sender == verticalResizePanel;
				HorizontalResizing = sender == resizePanel || sender == horizontalResizePanel;
			}
		}

		private void Resize_MouseUp(object sender, MouseEventArgs e)
		{
			if (WindowState is FormWindowState.Normal)
			{
				Resizing = false;
				((Control)sender).Capture = false;
			}
		}

		private void Resize_MouseMove(object sender, MouseEventArgs e)
		{
			if (WindowState is FormWindowState.Normal)
			{
				if (Resizing)
				{
					if (HorizontalResizing)
					{
						Width += e.X - ResizingMouseOrigin.X;
					}
					if (VerticalResizing)
					{
						Height += e.Y - ResizingMouseOrigin.Y;
					}
				}
			}
			else
			{
				Resizing = false;
				((Control)sender).Capture = false;
			}
		}

		#endregion

		#region Move

		bool Moving;
		(int X, int Y) MovingMouseOrigin;

		private void Move_MouseDown(object sender, MouseEventArgs e)
		{
			if (WindowState is FormWindowState.Normal)
			{
				Moving = true;
				movePanel.Capture = true;
				MovingMouseOrigin = (e.X, e.Y);
			}
		}

		private void Move_MouseUp(object sender, MouseEventArgs e)
		{
			if (WindowState is FormWindowState.Normal)
			{
				Moving = false;
				movePanel.Capture = false;
			}
		}

		private void Move_MouseMove(object sender, MouseEventArgs e)
		{
			if (WindowState is FormWindowState.Normal)
			{
				if (Moving)
				{
					Left += e.X - MovingMouseOrigin.X;
					Top += e.Y - MovingMouseOrigin.Y;
				}
			}
			else
			{
				Moving = false;
				movePanel.Capture = false;
			}
		}

		#endregion

		private void ClosePanel_Click(object sender, EventArgs e)
		{
			Close();
		}

		private void MaximizePanel_Click(object sender, EventArgs e)
		{
			if (WindowState is FormWindowState.Normal)
			{
				WindowState = FormWindowState.Maximized;
			}
			else if (WindowState is FormWindowState.Maximized)
			{
				WindowState = FormWindowState.Normal;
			}
		}

		private void MinimizePanel_Click(object sender, EventArgs e)
		{
			if (WindowState is FormWindowState.Normal)
			{
				WindowState = FormWindowState.Minimized;
			}
			else if (WindowState is FormWindowState.Minimized)
			{
				WindowState = FormWindowState.Normal;
			}
		}
	}
}
